#[cfg(feature = "ssr")]
use crate::backend::adguard::apply_adguard_lockdown;
#[cfg(feature = "ssr")]
use chrono::Utc;
#[cfg(feature = "ssr")]
use sqlx::{Pool, Sqlite};

pub struct AdguardConfig<'a> {
    pub enabled: bool,
    pub url: &'a str,
    pub username: &'a str,
    pub password: &'a str,
    pub client_id: &'a str,
}

#[cfg(feature = "ssr")]
pub async fn check_expired_lockdown(
    pool: &Pool<Sqlite>,
    uid: &str,
    locked_until: Option<&str>,
    adguard: &AdguardConfig<'_>,
) {
    if let Some(until_str) = locked_until {
        if let Ok(until_dt) = chrono::DateTime::parse_from_rfc3339(until_str) {
            if Utc::now() >= until_dt.with_timezone(&Utc) {
                let _ =
                    sqlx::query("UPDATE user_settings SET locked_until = NULL WHERE user_id = ?")
                        .bind(uid)
                        .execute(pool)
                        .await;
                if adguard.enabled {
                    let _ = apply_adguard_lockdown(
                        adguard.url,
                        adguard.username,
                        adguard.password,
                        adguard.client_id,
                        &[],
                        false,
                    )
                    .await;
                }
            }
        }
    }
}

#[cfg(feature = "ssr")]
pub async fn apply_missed_lockdown(
    pool: &Pool<Sqlite>,
    uid: &str,
    lock_until: Option<String>,
    adguard: &AdguardConfig<'_>,
    services_raw: &str,
) {
    if let Some(until_str) = lock_until {
        let _ = sqlx::query("UPDATE user_settings SET locked_until = ? WHERE user_id = ?")
            .bind(&until_str)
            .bind(uid)
            .execute(pool)
            .await;
        if adguard.enabled {
            let services: Vec<String> = services_raw
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            let _ = apply_adguard_lockdown(
                adguard.url,
                adguard.username,
                adguard.password,
                adguard.client_id,
                &services,
                true,
            )
            .await;
        }
    }
}
