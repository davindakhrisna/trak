#[cfg(feature = "ssr")]
#[tokio::test]
async fn test_validate_outbound_url_ssrf_protection() {
    use trak::backend::security::validate_outbound_url;

    // Reject non-http(s) schemes
    assert!(validate_outbound_url("ftp://example.com").is_err());
    assert!(validate_outbound_url("file:///etc/passwd").is_err());
    assert!(validate_outbound_url("gopher://localhost").is_err());

    // Reject cloud metadata endpoints
    assert!(validate_outbound_url("http://169.254.169.254/latest/meta-data").is_err());
    assert!(validate_outbound_url("http://metadata.google.internal/computeMetadata/v1").is_err());

    // Reject local loopback probes
    assert!(validate_outbound_url("http://127.0.0.1:3000").is_err());
    assert!(validate_outbound_url("http://localhost:8080").is_err());
    assert!(validate_outbound_url("http://0.0.0.0:80").is_err());
    assert!(validate_outbound_url("http://[::1]:9000").is_err());

    // Allow legitimate external & LAN URLs
    assert_eq!(
        validate_outbound_url("https://ntfy.sh").unwrap(),
        "https://ntfy.sh"
    );
    assert_eq!(
        validate_outbound_url("http://192.168.1.50:3000/").unwrap(),
        "http://192.168.1.50:3000"
    );
}

#[cfg(feature = "ssr")]
#[tokio::test]
async fn test_rate_limiting_ip_and_account_isolation() {
    use trak::backend::security::{
        check_login_rate_limit, clear_login_attempts, record_failed_login,
    };

    let attacker_ip = "198.51.100.42";
    let victim_ip = "203.0.113.19";
    let target_user = "admin";

    // Clean up state
    clear_login_attempts(attacker_ip, target_user).await;
    clear_login_attempts(victim_ip, target_user).await;

    // Attacker fails 5 times on target_user
    for _ in 0..5 {
        record_failed_login(attacker_ip, target_user).await;
    }

    // Attacker is locked out for target_user
    let attacker_res = check_login_rate_limit(attacker_ip, target_user).await;
    assert!(attacker_res.is_err());

    // Legitimate user on another IP is NOT locked out (no DoS)
    let victim_res = check_login_rate_limit(victim_ip, target_user).await;
    assert!(victim_res.is_ok());

    // Attacker fails on other usernames up to 10 total fails
    for i in 1..=5 {
        record_failed_login(attacker_ip, &format!("fake_user_{}", i)).await;
    }

    // Attacker IP is now globally blocked for ANY account
    let global_block = check_login_rate_limit(attacker_ip, "any_other_user").await;
    assert!(global_block.is_err());

    // Resetting clears the lock
    clear_login_attempts(attacker_ip, target_user).await;
}

#[cfg(feature = "ssr")]
#[tokio::test]
async fn test_database_single_first_admin_constraint() {
    use trak::backend::db::init_db;

    let pool = init_db("sqlite::memory:").await.unwrap();

    let u1 = "INSERT INTO users (id, username, password_hash, role, is_first_admin, timezone, created_at) \
              VALUES ('u1', 'admin1', 'hash1', 'ADMIN', 1, 'UTC', '2026-01-01T00:00:00Z')";
    let res1 = sqlx::query(u1).execute(&pool).await;
    assert!(res1.is_ok());

    // Second user with is_first_admin = 0 is fine
    let u2 = "INSERT INTO users (id, username, password_hash, role, is_first_admin, timezone, created_at) \
              VALUES ('u2', 'user2', 'hash2', 'USER', 0, 'UTC', '2026-01-01T00:00:00Z')";
    let res2 = sqlx::query(u2).execute(&pool).await;
    assert!(res2.is_ok());

    // Second user attempting is_first_admin = 1 MUST be rejected by SQLite engine index
    let u3 = "INSERT INTO users (id, username, password_hash, role, is_first_admin, timezone, created_at) \
              VALUES ('u3', 'hacker', 'hash3', 'ADMIN', 1, 'UTC', '2026-01-01T00:00:00Z')";
    let res3 = sqlx::query(u3).execute(&pool).await;
    assert!(res3.is_err(), "Engine must reject duplicate first admin");
}

#[cfg(feature = "ssr")]
#[test]
fn test_token_verification_and_tamper_defense() {
    use trak::backend::auth::{sign_token, verify_token, AuthClaims};
    use trak::domain::UserRole;

    let secret = b"super-secure-production-hmac-key-256";
    let claims = AuthClaims::new("user-1".to_string(), "alice".to_string(), UserRole::User);

    let token = sign_token(&claims, secret).unwrap();
    assert!(verify_token(&token, secret).is_some());

    // Tampering with payload fails HMAC check
    let parts: Vec<&str> = token.split('.').collect();
    let tampered_payload =
        hex::encode(r#"{"user_id":"user-1","username":"alice","role":"ADMIN","exp":9999999999}"#);
    let forged_token = format!("{}.{}", tampered_payload, parts[1]);
    assert!(verify_token(&forged_token, secret).is_none());

    // Wrong secret fails verification
    assert!(verify_token(&token, b"wrong-hmac-secret-key").is_none());
}
