#[cfg(feature = "ssr")]
use crate::backend::AppState;
#[cfg(feature = "ssr")]
use crate::domain::LockdownStatus;
#[cfg(feature = "ssr")]
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
#[cfg(feature = "ssr")]
use chrono::Utc;
#[cfg(feature = "ssr")]
use sqlx::Row;

#[cfg(feature = "ssr")]
pub async fn get_lockdown_status_handler(
    State(state): State<AppState>,
    Path(secret): Path<String>,
) -> Result<Json<LockdownStatus>, StatusCode> {
    let row = sqlx::query(
        "SELECT user_id, lockdown_enabled, locked_until FROM user_settings WHERE lockdown_secret = ?",
    )
    .bind(&secret)
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let r = match row {
        Some(r) => r,
        None => return Err(StatusCode::NOT_FOUND),
    };

    let l_on: i64 = r.get("lockdown_enabled");
    let locked_until: Option<String> = r.get("locked_until");

    if l_on == 0 || locked_until.is_none() {
        return Ok(Json(LockdownStatus {
            locked: false,
            locked_until: None,
            missed_deadlines: 0,
            reason: "Lockdown is inactive or not triggered.".to_string(),
        }));
    }

    let until_str = locked_until.unwrap();
    let is_locked = if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&until_str) {
        Utc::now() < dt
    } else {
        false
    };

    if is_locked {
        Ok(Json(LockdownStatus {
            locked: true,
            locked_until: Some(until_str.clone()),
            missed_deadlines: 1,
            reason: format!("TRAK LOCKDOWN ACTIVE until {}", until_str),
        }))
    } else {
        Ok(Json(LockdownStatus {
            locked: false,
            locked_until: None,
            missed_deadlines: 0,
            reason: "Previous lockdown has expired.".to_string(),
        }))
    }
}
