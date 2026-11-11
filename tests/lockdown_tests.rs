#[cfg(feature = "ssr")]
mod tests {
    use axum::extract::{Path, State};
    use chrono::{Duration, Utc};
    use sqlx::Row;
    use trak::backend::db::init_db;
    use trak::backend::lockdown_api::get_lockdown_status_handler;
    use trak::backend::lockdown_enforcer::{apply_missed_lockdown, check_expired_lockdown};
    use trak::backend::security::{cache_lockdown, get_cached_lockdown};
    use trak::backend::AppState;

    #[tokio::test]
    async fn test_lockdown_secret_format_validation() {
        let pool = init_db("sqlite::memory:").await.unwrap();
        let state = AppState {
            pool,
            secret: vec![0u8; 32],
            leptos_options: leptos::config::LeptosOptions::builder()
                .output_name("trak")
                .site_root("target/site")
                .site_pkg_dir("pkg")
                .build(),
        };

        // Non-32 hex characters must return 404 Not Found immediately
        assert_eq!(
            get_lockdown_status_handler(State(state.clone()), Path("short".to_string()))
                .await
                .unwrap_err(),
            axum::http::StatusCode::NOT_FOUND
        );

        assert_eq!(
            get_lockdown_status_handler(
                State(state.clone()),
                Path("0123456789abcdef0123456789abcdef01".to_string())
            )
            .await
            .unwrap_err(),
            axum::http::StatusCode::NOT_FOUND
        );

        assert_eq!(
            get_lockdown_status_handler(
                State(state),
                Path("zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz".to_string())
            )
            .await
            .unwrap_err(),
            axum::http::StatusCode::NOT_FOUND
        );
    }

    #[tokio::test]
    async fn test_lockdown_endpoint_inactive_and_active_status() {
        let pool = init_db("sqlite::memory:").await.unwrap();
        let valid_secret = "0123456789abcdef0123456789abcdef";

        sqlx::query("INSERT INTO users (id, username, password_hash, role, is_first_admin, timezone, created_at) VALUES ('u1', 'testadmin', 'h', 'ADMIN', 1, 'UTC', '2026-01-01T00:00:00Z')")
            .execute(&pool)
            .await
            .unwrap();

        sqlx::query("INSERT INTO user_settings (user_id, ntfy_enabled, ntfy_server, ntfy_topic, lockdown_enabled, lockdown_secret, locked_until) VALUES ('u1', 0, 'https://ntfy.sh', '', 0, ?, NULL)")
            .bind(valid_secret)
            .execute(&pool)
            .await
            .unwrap();

        let state = AppState {
            pool: pool.clone(),
            secret: vec![0u8; 32],
            leptos_options: leptos::config::LeptosOptions::builder()
                .output_name("trak")
                .site_root("target/site")
                .site_pkg_dir("pkg")
                .build(),
        };

        // 1. Inactive lockdown returns locked = false
        let res1 =
            get_lockdown_status_handler(State(state.clone()), Path(valid_secret.to_string()))
                .await
                .unwrap();
        assert!(!res1.0.locked, "Lockdown should be inactive");
        assert_eq!(res1.0.missed_deadlines, 0);

        // 2. Active lockdown with locked_until in the future
        let future_lock = (Utc::now() + Duration::hours(2)).to_rfc3339();
        sqlx::query(
            "UPDATE user_settings SET lockdown_enabled = 1, locked_until = ? WHERE user_id = 'u1'",
        )
        .bind(&future_lock)
        .execute(&pool)
        .await
        .unwrap();

        cache_lockdown(
            valid_secret,
            &trak::domain::LockdownStatus {
                locked: true,
                locked_until: Some(future_lock.clone()),
                missed_deadlines: 1,
                reason: format!("TRAK LOCKDOWN ACTIVE until {}", future_lock),
            },
        )
        .await;

        let res2 = get_lockdown_status_handler(State(state), Path(valid_secret.to_string()))
            .await
            .unwrap();
        assert!(
            res2.0.locked,
            "Lockdown must be active when locked_until is future"
        );
        assert_eq!(res2.0.missed_deadlines, 1);
        assert_eq!(res2.0.locked_until, Some(future_lock));
    }

    #[tokio::test]
    async fn test_lockdown_enforcer_expiration_and_application() {
        let pool = init_db("sqlite::memory:").await.unwrap();

        sqlx::query("INSERT INTO users (id, username, password_hash, role, is_first_admin, timezone, created_at) VALUES ('u2', 'user2', 'h', 'USER', 0, 'UTC', '2026-01-01T00:00:00Z')")
            .execute(&pool)
            .await
            .unwrap();

        sqlx::query("INSERT INTO user_settings (user_id, ntfy_enabled, ntfy_server, ntfy_topic, lockdown_enabled, lockdown_secret, locked_until) VALUES ('u2', 0, 'https://ntfy.sh', '', 1, 'aabbccddeeff00112233445566778899', NULL)")
            .execute(&pool)
            .await
            .unwrap();

        // Apply missed lockdown
        let lock_time = (Utc::now() + Duration::minutes(30)).to_rfc3339();
        apply_missed_lockdown(&pool, "u2", Some(lock_time.clone())).await;

        let row1 = sqlx::query("SELECT locked_until FROM user_settings WHERE user_id = 'u2'")
            .fetch_one(&pool)
            .await
            .unwrap();
        let val1: Option<String> = row1.get("locked_until");
        assert_eq!(val1, Some(lock_time));

        // Check expired lockdown with a past timestamp
        let past_time = (Utc::now() - Duration::minutes(5)).to_rfc3339();
        check_expired_lockdown(&pool, "u2", Some(&past_time)).await;

        let row2 = sqlx::query("SELECT locked_until FROM user_settings WHERE user_id = 'u2'")
            .fetch_one(&pool)
            .await
            .unwrap();
        let val2: Option<String> = row2.get("locked_until");
        assert_eq!(val2, None, "Expired lockdown must be cleared to NULL");
    }

    #[tokio::test]
    async fn test_lockdown_in_memory_cache() {
        let test_secret = "cafecafecafecafecafecafecafecafe";
        assert!(get_cached_lockdown(test_secret).await.is_none());

        let status = trak::domain::LockdownStatus {
            locked: true,
            locked_until: Some("2026-11-11T20:00:00Z".to_string()),
            missed_deadlines: 2,
            reason: "Lockdown test".to_string(),
        };

        cache_lockdown(test_secret, &status).await;
        let cached = get_cached_lockdown(test_secret).await;
        assert!(
            cached.is_some(),
            "Lockdown status should be cached in memory"
        );
        assert_eq!(cached.unwrap().missed_deadlines, 2);
    }
}
