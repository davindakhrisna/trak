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
    use crate::backend::AppState;

    let state = expect_context::<AppState>();
    let claims =
        get_auth_claims_from_context(&state).ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    sqlx::query(
        r#"
        UPDATE user_settings
        SET ntfy_enabled = ?, ntfy_server = ?, ntfy_topic = ?,
            notify_start = ?, notify_1h = ?, notify_25m = ?, notify_crossed = ?,
            lockdown_enabled = ?, timezone = ?,
            adguard_enabled = ?, adguard_url = ?, adguard_username = ?,
            adguard_password = ?, adguard_client_id = ?, adguard_services = ?
        WHERE user_id = ?
        "#,
    )
    .bind(if settings.ntfy_enabled { 1 } else { 0 })
    .bind(settings.ntfy_server.trim())
    .bind(settings.ntfy_topic.trim())
    .bind(if settings.notify_start { 1 } else { 0 })
    .bind(if settings.notify_1h { 1 } else { 0 })
    .bind(if settings.notify_25m { 1 } else { 0 })
    .bind(if settings.notify_crossed { 1 } else { 0 })
    .bind(if settings.lockdown_enabled { 1 } else { 0 })
    .bind(settings.timezone.trim())
    .bind(if settings.adguard_enabled { 1 } else { 0 })
    .bind(settings.adguard_url.trim())
    .bind(settings.adguard_username.trim())
    .bind(settings.adguard_password.trim())
    .bind(settings.adguard_client_id.trim())
    .bind(settings.adguard_services.trim())
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
    use crate::backend::AppState;

    let state = expect_context::<AppState>();
    let claims =
        get_auth_claims_from_context(&state).ok_or_else(|| ServerFnError::new("Unauthorized"))?;

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
pub async fn test_adguard(
    url: Option<String>,
    username: Option<String>,
    password: Option<String>,
) -> Result<String, ServerFnError> {
    use crate::backend::adguard::test_adguard_connection;
    use crate::backend::auth::get_auth_claims_from_context;
    use crate::backend::settings_db::ensure_user_settings;
    use crate::backend::AppState;

    let state = expect_context::<AppState>();
    let claims =
        get_auth_claims_from_context(&state).ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    let (u, user, pass) = match (url, username, password) {
        (Some(u), Some(user), Some(pass)) if !u.trim().is_empty() => (u, user, pass),
        _ => {
            let settings = ensure_user_settings(&state.pool, &claims.user_id)
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))?;
            (
                settings.adguard_url,
                settings.adguard_username,
                settings.adguard_password,
            )
        }
    };

    test_adguard_connection(&u, &user, &pass)
        .await
        .map_err(ServerFnError::new)
}

#[server]
pub async fn emergency_reset_lockdown() -> Result<(), ServerFnError> {
    use crate::backend::adguard::apply_adguard_lockdown;
    use crate::backend::auth::get_auth_claims_from_context;
    use crate::backend::settings_db::ensure_user_settings;
    use crate::backend::AppState;

    let state = expect_context::<AppState>();
    let claims =
        get_auth_claims_from_context(&state).ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    let settings = ensure_user_settings(&state.pool, &claims.user_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    if settings.adguard_enabled {
        let _ = apply_adguard_lockdown(
            &settings.adguard_url,
            &settings.adguard_username,
            &settings.adguard_password,
            &settings.adguard_client_id,
            &[],
            false,
        )
        .await;
    }

    sqlx::query("UPDATE user_settings SET locked_until = NULL WHERE user_id = ?")
        .bind(&claims.user_id)
        .execute(&state.pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}
