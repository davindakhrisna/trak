#[cfg(feature = "ssr")]
use crate::domain::LockdownStatus;
#[cfg(feature = "ssr")]
use axum::{extract::Request, middleware::Next, response::Response};
#[cfg(feature = "ssr")]
use std::collections::HashMap;
#[cfg(feature = "ssr")]
use std::sync::LazyLock;
#[cfg(feature = "ssr")]
use std::time::{Duration, Instant};
#[cfg(feature = "ssr")]
use tokio::sync::Mutex;

#[cfg(feature = "ssr")]
static LOGIN_ATTEMPTS: LazyLock<Mutex<HashMap<String, (u32, Instant)>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[cfg(feature = "ssr")]
static LOCKDOWN_CACHE: LazyLock<Mutex<HashMap<String, (Instant, LockdownStatus)>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[cfg(feature = "ssr")]
pub fn get_client_ip() -> String {
    if let Some(parts) = leptos::prelude::use_context::<http::request::Parts>() {
        if let Some(xff) = parts
            .headers
            .get("x-forwarded-for")
            .and_then(|v| v.to_str().ok())
        {
            if let Some(client_ip) = xff.split(',').next() {
                let trimmed = client_ip.trim();
                if !trimmed.is_empty() {
                    return trimmed.to_string();
                }
            }
        }
        if let Some(xri) = parts.headers.get("x-real-ip").and_then(|v| v.to_str().ok()) {
            let trimmed = xri.trim();
            if !trimmed.is_empty() {
                return trimmed.to_string();
            }
        }
    }
    "127.0.0.1".to_string()
}

#[cfg(feature = "ssr")]
pub async fn check_login_rate_limit(ip: &str, username: &str) -> Result<(), &'static str> {
    let mut map = LOGIN_ATTEMPTS.lock().await;
    let now = Instant::now();
    map.retain(|_, (_, time)| now.duration_since(*time) < Duration::from_secs(300));

    let ip_key = format!("ip:{}", ip);
    if let Some((fails, _)) = map.get(&ip_key) {
        if *fails >= 10 {
            return Err("Too many failed attempts from your IP. Please wait 5 minutes.");
        }
    }

    let user_key = format!("usr:{}:{}", ip, username);
    if let Some((fails, _)) = map.get(&user_key) {
        if *fails >= 5 {
            return Err("Too many failed attempts for this account. Please wait 5 minutes.");
        }
    }

    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn record_failed_login(ip: &str, username: &str) {
    let mut map = LOGIN_ATTEMPTS.lock().await;
    let now = Instant::now();
    let ip_key = format!("ip:{}", ip);
    let user_key = format!("usr:{}:{}", ip, username);

    let ip_entry = map.entry(ip_key).or_insert((0, now));
    ip_entry.0 += 1;
    ip_entry.1 = now;

    let user_entry = map.entry(user_key).or_insert((0, now));
    user_entry.0 += 1;
    user_entry.1 = now;
}

#[cfg(feature = "ssr")]
pub async fn clear_login_attempts(ip: &str, username: &str) {
    let mut map = LOGIN_ATTEMPTS.lock().await;
    map.remove(&format!("ip:{}", ip));
    map.remove(&format!("usr:{}:{}", ip, username));
}

#[cfg(feature = "ssr")]
pub async fn get_cached_lockdown(secret: &str) -> Option<LockdownStatus> {
    let map = LOCKDOWN_CACHE.lock().await;
    if let Some((time, status)) = map.get(secret) {
        if time.elapsed() < Duration::from_secs(5) {
            return Some(status.clone());
        }
    }
    None
}

#[cfg(feature = "ssr")]
pub async fn cache_lockdown(secret: &str, status: &LockdownStatus) {
    let mut map = LOCKDOWN_CACHE.lock().await;
    let now = Instant::now();
    map.retain(|_, (time, _)| now.duration_since(*time) < Duration::from_secs(60));
    map.insert(secret.to_string(), (now, status.clone()));
}

#[cfg(feature = "ssr")]
pub async fn invalidate_lockdown_cache(secret: &str) {
    let mut map = LOCKDOWN_CACHE.lock().await;
    map.remove(secret);
}

#[cfg(feature = "ssr")]
pub fn validate_outbound_url(url: &str) -> Result<String, String> {
    let clean = url.trim().trim_end_matches('/');
    if clean.is_empty() {
        return Err("URL cannot be empty.".to_string());
    }
    if !clean.starts_with("http://") && !clean.starts_with("https://") {
        return Err("URL must start with http:// or https://".to_string());
    }
    let lower = clean.to_lowercase();
    if lower.contains("169.254.169.254") || lower.contains("metadata.google.internal") {
        return Err("Access to cloud metadata endpoints is prohibited.".to_string());
    }
    if lower.contains("127.0.0.1")
        || lower.contains("localhost")
        || lower.contains("0.0.0.0")
        || lower.contains("[::1]")
    {
        return Err("Access to local loopback addresses is prohibited.".to_string());
    }
    Ok(clean.to_string())
}

#[cfg(feature = "ssr")]
pub async fn apply_security_headers(req: Request, next: Next) -> Response {
    let mut res = next.run(req).await;
    let headers = res.headers_mut();
    headers.insert(
        http::header::X_CONTENT_TYPE_OPTIONS,
        http::HeaderValue::from_static("nosniff"),
    );
    headers.insert(
        http::header::X_FRAME_OPTIONS,
        http::HeaderValue::from_static("DENY"),
    );
    headers.insert(
        http::header::REFERRER_POLICY,
        http::HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
    if let Ok(val) = http::HeaderValue::from_str("camera=(), microphone=(), geolocation=()") {
        headers.insert(
            http::header::HeaderName::from_static("permissions-policy"),
            val,
        );
    }
    res
}
