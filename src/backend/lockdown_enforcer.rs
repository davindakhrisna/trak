#[cfg(feature = "ssr")]
use chrono::Utc;
#[cfg(feature = "ssr")]
use sqlx::{Pool, Sqlite};

#[cfg(feature = "ssr")]
pub async fn check_expired_lockdown(pool: &Pool<Sqlite>, uid: &str, locked_until: Option<&str>) {
    if let Some(until_str) = locked_until {
        if let Ok(until_dt) = chrono::DateTime::parse_from_rfc3339(until_str) {
            if Utc::now() >= until_dt.with_timezone(&Utc) {
                let _ =
                    sqlx::query("UPDATE user_settings SET locked_until = NULL WHERE user_id = ?")
                        .bind(uid)
                        .execute(pool)
                        .await;
            }
        }
    }
}

#[cfg(feature = "ssr")]
pub async fn apply_missed_lockdown(pool: &Pool<Sqlite>, uid: &str, lock_until: Option<String>) {
    if let Some(until_str) = lock_until {
        let _ = sqlx::query("UPDATE user_settings SET locked_until = ? WHERE user_id = ?")
            .bind(&until_str)
            .bind(uid)
            .execute(pool)
            .await;
    }
}
