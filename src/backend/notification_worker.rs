#[cfg(feature = "ssr")]
use crate::backend::lockdown_enforcer::{
    apply_missed_lockdown, check_expired_lockdown, AdguardConfig,
};
#[cfg(feature = "ssr")]
use crate::backend::notification_task_checker::{check_user_tasks, NotifyFlags};
#[cfg(feature = "ssr")]
use chrono::Utc;
#[cfg(feature = "ssr")]
use sqlx::{Pool, Row, Sqlite};
#[cfg(feature = "ssr")]
use std::time::Duration;

#[cfg(feature = "ssr")]
pub fn spawn_notification_ticker(pool: Pool<Sqlite>) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(30));
        loop {
            interval.tick().await;
            let _ = process_notifications(&pool).await;
        }
    });
}

#[cfg(feature = "ssr")]
async fn process_notifications(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    let users = sqlx::query(
        "SELECT u.id, s.ntfy_enabled, s.ntfy_server, s.ntfy_topic, \
                s.notify_start, s.notify_1h, s.notify_25m, s.notify_crossed, \
                s.lockdown_enabled, s.timezone, s.adguard_enabled, s.adguard_url, \
                s.adguard_username, s.adguard_password, s.adguard_client_id, \
                s.adguard_services, s.locked_until \
         FROM users u JOIN user_settings s ON u.id = s.user_id",
    )
    .fetch_all(pool)
    .await?;

    for u in users {
        let (uid, ntfy_on, server, topic): (String, i64, String, String) = (
            u.get("id"),
            u.get("ntfy_enabled"),
            u.get("ntfy_server"),
            u.get("ntfy_topic"),
        );
        let (n_start, n_1h, n_25m, n_cross): (i64, i64, i64, i64) = (
            u.get("notify_start"),
            u.get("notify_1h"),
            u.get("notify_25m"),
            u.get("notify_crossed"),
        );
        let (lockdown_on, tz_str): (i64, String) = (u.get("lockdown_enabled"), u.get("timezone"));
        let (adguard_on, adguard_url, adguard_user, adguard_pass, client_id, services_raw): (
            i64,
            String,
            String,
            String,
            String,
            String,
        ) = (
            u.get("adguard_enabled"),
            u.get("adguard_url"),
            u.get("adguard_username"),
            u.get("adguard_password"),
            u.get("adguard_client_id"),
            u.get("adguard_services"),
        );
        let current_locked_until: Option<String> = u.get("locked_until");

        let tz = tz_str.parse::<chrono_tz::Tz>().unwrap_or(chrono_tz::UTC);
        let now_local = Utc::now().with_timezone(&tz);

        let adguard = AdguardConfig {
            enabled: adguard_on == 1,
            url: &adguard_url,
            username: &adguard_user,
            password: &adguard_pass,
            client_id: &client_id,
        };

        check_expired_lockdown(pool, &uid, current_locked_until.as_deref(), &adguard).await;

        let flags = NotifyFlags {
            ntfy_on: ntfy_on == 1,
            n_start: n_start == 1,
            n_1h: n_1h == 1,
            n_25m: n_25m == 1,
            n_cross: n_cross == 1,
        };

        let missed_count =
            check_user_tasks(pool, &uid, &server, &topic, tz, now_local, &flags).await;

        if lockdown_on == 1 && missed_count > 0 {
            let lock_until = if missed_count >= 3 {
                now_local
                    .date_naive()
                    .succ_opt()
                    .and_then(|d| d.and_hms_opt(23, 59, 59))
                    .and_then(|dt| dt.and_local_timezone(tz).single())
                    .map(|t| t.to_rfc3339())
            } else {
                now_local
                    .date_naive()
                    .and_hms_opt(23, 59, 59)
                    .and_then(|dt| dt.and_local_timezone(tz).single())
                    .map(|t| t.to_rfc3339())
            };
            apply_missed_lockdown(pool, &uid, lock_until, &adguard, &services_raw).await;
        }
    }
    Ok(())
}
