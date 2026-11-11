use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use crate::domain::UserRole;
#[cfg(feature = "ssr")]
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
#[cfg(feature = "ssr")]
use hmac::{Hmac, Mac};
#[cfg(feature = "ssr")]
use sha2::Sha256;

#[cfg(feature = "ssr")]
type HmacSha256 = Hmac<Sha256>;

pub const SESSION_COOKIE_NAME: &str = "trak_session";
pub const SESSION_DURATION_SECS: i64 = 30 * 24 * 3600; // 30 days

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthClaims {
    pub user_id: String,
    pub username: String,
    pub role: String,
    pub exp: i64,
}

#[cfg(feature = "ssr")]
pub fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| format!("Password hashing failed: {}", e))
}

#[cfg(feature = "ssr")]
pub fn verify_password(password: &str, hash: &str) -> bool {
    let parsed_hash = match PasswordHash::new(hash) {
        Ok(h) => h,
        Err(_) => return false,
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

#[cfg(feature = "ssr")]
pub fn sign_token(claims: &AuthClaims, secret: &[u8]) -> Result<String, String> {
    let payload =
        serde_json::to_string(claims).map_err(|e| format!("Serialization error: {}", e))?;
    let payload_b64 = hex::encode(payload.as_bytes());

    let mut mac =
        HmacSha256::new_from_slice(secret).map_err(|e| format!("HMAC key error: {}", e))?;
    mac.update(payload_b64.as_bytes());
    let signature = hex::encode(mac.finalize().into_bytes());

    Ok(format!("{}.{}", payload_b64, signature))
}

#[cfg(feature = "ssr")]
pub fn verify_token(token: &str, secret: &[u8]) -> Option<AuthClaims> {
    let mut parts = token.split('.');
    let payload_hex = parts.next()?;
    let signature_hex = parts.next()?;
    if parts.next().is_some() {
        return None;
    }

    let mut mac = HmacSha256::new_from_slice(secret).ok()?;
    mac.update(payload_hex.as_bytes());
    let sig_bytes = hex::decode(signature_hex).ok()?;
    if mac.verify_slice(&sig_bytes).is_err() {
        return None;
    }

    let payload_bytes = hex::decode(payload_hex).ok()?;
    let claims: AuthClaims = serde_json::from_slice(&payload_bytes).ok()?;

    let now = chrono::Utc::now().timestamp();
    if claims.exp < now {
        return None;
    }

    Some(claims)
}

#[cfg(feature = "ssr")]
impl AuthClaims {
    pub fn new(user_id: String, username: String, role: UserRole) -> Self {
        let exp = chrono::Utc::now().timestamp() + SESSION_DURATION_SECS;
        Self {
            user_id,
            username,
            role: role.label().to_string(),
            exp,
        }
    }

    pub fn is_admin(&self) -> bool {
        self.role == "ADMIN"
    }
}

#[cfg(feature = "ssr")]
pub fn get_auth_claims_from_context(state: &crate::backend::AppState) -> Option<AuthClaims> {
    use http::header::COOKIE;
    let parts = leptos::prelude::use_context::<http::request::Parts>()?;
    let cookie_header = parts.headers.get(COOKIE)?.to_str().ok()?;
    for pair in cookie_header.split(';') {
        let mut kv = pair.trim().split('=');
        if let (Some(k), Some(v)) = (kv.next(), kv.next()) {
            if k == SESSION_COOKIE_NAME {
                return verify_token(v, &state.secret);
            }
        }
    }
    None
}

#[cfg(feature = "ssr")]
fn is_secure_request() -> bool {
    if let Some(parts) = leptos::prelude::use_context::<http::request::Parts>() {
        if let Some(proto) = parts
            .headers
            .get("x-forwarded-proto")
            .and_then(|v| v.to_str().ok())
        {
            if proto.eq_ignore_ascii_case("https") {
                return true;
            }
        }
    }
    std::env::var("APP_ENV").as_deref() == Ok("production")
        || std::env::var("SECURE_COOKIES").as_deref() == Ok("true")
}

#[cfg(feature = "ssr")]
pub fn set_session_cookie(token: &str) {
    use http::header::{HeaderValue, SET_COOKIE};
    if let Some(res_opts) = leptos::prelude::use_context::<leptos_axum::ResponseOptions>() {
        let sec_flag = if is_secure_request() { "; Secure" } else { "" };
        let cookie_val = format!(
            "{}={}; Path=/; HttpOnly; SameSite=Lax; Max-Age={}{}",
            SESSION_COOKIE_NAME, token, SESSION_DURATION_SECS, sec_flag
        );
        if let Ok(hv) = HeaderValue::from_str(&cookie_val) {
            res_opts.append_header(SET_COOKIE, hv);
        }
    }
}

#[cfg(feature = "ssr")]
pub fn clear_session_cookie() {
    use http::header::{HeaderValue, SET_COOKIE};
    if let Some(res_opts) = leptos::prelude::use_context::<leptos_axum::ResponseOptions>() {
        let sec_flag = if is_secure_request() { "; Secure" } else { "" };
        let cookie_val = format!(
            "{}=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0{}",
            SESSION_COOKIE_NAME, sec_flag
        );
        if let Ok(hv) = HeaderValue::from_str(&cookie_val) {
            res_opts.append_header(SET_COOKIE, hv);
        }
    }
}
