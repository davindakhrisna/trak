use crate::domain::UserSettings;
use leptos::prelude::*;

#[server]
pub async fn fetch_settings() -> Result<UserSettings, ServerFnError> {
    use crate::backend::auth::get_auth_claims_from_context;
    use crate::backend::settings_db::ensure_user_settings;
    use crate::backend::AppState;

    let state = expect_context::<AppState>();
    let claims =
        get_auth_claims_from_context(&state).ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    let settings = ensure_user_settings(&state.pool, &claims.user_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(settings)
}

#[server]
pub async fn update_settings(settings: UserSettings) -> Result<UserSettings, ServerFnError> {
    use crate::backend::auth::get_auth_claims_from_context;
    use crate::backend::security::{invalidate_lockdown_cache, validate_outbound_url};
    use crate::backend::AppState;

    let state = expect_context::<AppState>();
    let claims =
        get_auth_claims_from_context(&state).ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    let clean_tz = settings.timezone.trim();
    clean_tz
        .parse::<chrono_tz::Tz>()
        .map_err(|_| ServerFnError::new("Invalid IANA timezone name (e.g. UTC, Asia/Jakarta)."))?;

    validate_outbound_url(&settings.ntfy_server).map_err(ServerFnError::new)?;

    let clean_topic = settings.ntfy_topic.trim();
    if clean_topic.is_empty()
        || clean_topic.len() > 64
        || !clean_topic
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        return Err(ServerFnError::new(
            "Ntfy topic must be 1-64 characters (letters, numbers, dash, underscore).",
        ));
    }

    invalidate_lockdown_cache(&settings.lockdown_secret).await;

    sqlx::query(
        "UPDATE user_settings SET ntfy_enabled = ?, ntfy_server = ?, ntfy_topic = ?, \
         notify_start = ?, notify_1h = ?, notify_25m = ?, notify_crossed = ?, \
         lockdown_enabled = ?, timezone = ? \
         WHERE user_id = ?",
    )
    .bind(if settings.ntfy_enabled { 1 } else { 0 })
    .bind(settings.ntfy_server.trim())
    .bind(clean_topic)
    .bind(if settings.notify_start { 1 } else { 0 })
    .bind(if settings.notify_1h { 1 } else { 0 })
    .bind(if settings.notify_25m { 1 } else { 0 })
    .bind(if settings.notify_crossed { 1 } else { 0 })
    .bind(if settings.lockdown_enabled { 1 } else { 0 })
    .bind(clean_tz)
    .bind(&claims.user_id)
    .execute(&state.pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(settings)
}

#[server]
pub async fn regenerate_ntfy_topic() -> Result<String, ServerFnError> {
    use crate::backend::auth::get_auth_claims_from_context;
    use crate::backend::AppState;

    let state = expect_context::<AppState>();
    let claims =
        get_auth_claims_from_context(&state).ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    let rand_hex = uuid::Uuid::new_v4().to_string().replace('-', "");
    let new_topic = format!("trak-u-{}", &rand_hex[..12]);

    sqlx::query("UPDATE user_settings SET ntfy_topic = ? WHERE user_id = ?")
        .bind(&new_topic)
        .bind(&claims.user_id)
        .execute(&state.pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(new_topic)
}

#[server]
pub async fn regenerate_lockdown_secret() -> Result<String, ServerFnError> {
    use crate::backend::auth::get_auth_claims_from_context;
    use crate::backend::security::invalidate_lockdown_cache;
    use crate::backend::AppState;
    use sqlx::Row;

    let state = expect_context::<AppState>();
    let claims =
        get_auth_claims_from_context(&state).ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    let old_row = sqlx::query("SELECT lockdown_secret FROM user_settings WHERE user_id = ?")
        .bind(&claims.user_id)
        .fetch_optional(&state.pool)
        .await
        .ok()
        .flatten();

    if let Some(r) = old_row {
        let old_secret: String = r.get("lockdown_secret");
        invalidate_lockdown_cache(&old_secret).await;
    }

    let new_secret = uuid::Uuid::new_v4().to_string().replace('-', "");

    sqlx::query("UPDATE user_settings SET lockdown_secret = ? WHERE user_id = ?")
        .bind(&new_secret)
        .bind(&claims.user_id)
        .execute(&state.pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(new_secret)
}

#[server]
pub async fn send_test_ntfy() -> Result<(), ServerFnError> {
    use crate::backend::auth::get_auth_claims_from_context;
    use crate::backend::ntfy::send_ntfy_notification;
    use crate::backend::settings_db::ensure_user_settings;
    use crate::backend::AppState;

    let state = expect_context::<AppState>();
    let claims =
        get_auth_claims_from_context(&state).ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    let settings = ensure_user_settings(&state.pool, &claims.user_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    send_ntfy_notification(
        &settings.ntfy_server,
        &settings.ntfy_topic,
        "Trak: Test Notification",
        "Trak ntfy connection verified! Task alerts and discipline reminders are active.",
        3,
        &["white_check_mark", "tada"],
    )
    .await
    .map_err(ServerFnError::new)?;

    Ok(())
}

#[server]
pub async fn emergency_reset_lockdown() -> Result<(), ServerFnError> {
    use crate::backend::auth::get_auth_claims_from_context;
    use crate::backend::security::invalidate_lockdown_cache;
    use crate::backend::settings_db::ensure_user_settings;
    use crate::backend::AppState;

    let state = expect_context::<AppState>();
    let claims =
        get_auth_claims_from_context(&state).ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    let settings = ensure_user_settings(&state.pool, &claims.user_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    invalidate_lockdown_cache(&settings.lockdown_secret).await;

    sqlx::query("UPDATE user_settings SET locked_until = NULL WHERE user_id = ?")
        .bind(&claims.user_id)
        .execute(&state.pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}
