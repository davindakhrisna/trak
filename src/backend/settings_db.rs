#[cfg(feature = "ssr")]
use crate::domain::UserSettings;
#[cfg(feature = "ssr")]
use sqlx::{Pool, Row, Sqlite};

#[cfg(feature = "ssr")]
pub async fn ensure_user_settings(
    pool: &Pool<Sqlite>,
    uid: &str,
) -> Result<UserSettings, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT user_id, ntfy_enabled, ntfy_server, ntfy_topic,
               notify_start, notify_1h, notify_25m, notify_crossed,
               lockdown_enabled, lockdown_secret, locked_until, timezone,
               adguard_enabled, adguard_url, adguard_username, adguard_password,
               adguard_client_id, adguard_services
        FROM user_settings WHERE user_id = ?
        "#,
    )
    .bind(uid)
    .fetch_optional(pool)
    .await?;

    if let Some(r) = row {
        let n_on: i64 = r.get("ntfy_enabled");
        let n_st: i64 = r.get("notify_start");
        let n_1h: i64 = r.get("notify_1h");
        let n_25m: i64 = r.get("notify_25m");
        let n_cr: i64 = r.get("notify_crossed");
        let l_on: i64 = r.get("lockdown_enabled");
        let ag_on: i64 = r.try_get("adguard_enabled").unwrap_or(0);
        return Ok(UserSettings {
            user_id: r.get("user_id"),
            ntfy_enabled: n_on == 1,
            ntfy_server: r.get("ntfy_server"),
            ntfy_topic: r.get("ntfy_topic"),
            notify_start: n_st == 1,
            notify_1h: n_1h == 1,
            notify_25m: n_25m == 1,
            notify_crossed: n_cr == 1,
            lockdown_enabled: l_on == 1,
            lockdown_secret: r.get("lockdown_secret"),
            locked_until: r.get("locked_until"),
            missed_deadlines_today: 0,
            timezone: r.get("timezone"),
            adguard_enabled: ag_on == 1,
            adguard_url: r.try_get("adguard_url").unwrap_or_default(),
            adguard_username: r.try_get("adguard_username").unwrap_or_default(),
            adguard_password: r.try_get("adguard_password").unwrap_or_default(),
            adguard_client_id: r.try_get("adguard_client_id").unwrap_or_default(),
            adguard_services: r
                .try_get("adguard_services")
                .unwrap_or_else(|_| "instagram,tiktok,twitter,reddit,youtube".to_string()),
        });
    }

    let secret = uuid::Uuid::new_v4().to_string().replace('-', "");
    let topic = format!("trak-u-{}", &secret[..12]);
    let server = "https://ntfy.sh".to_string();
    let tz = "UTC".to_string();
    let def_svc = "instagram,tiktok,twitter,reddit,youtube".to_string();

    sqlx::query(
        r#"
        INSERT INTO user_settings (
            user_id, ntfy_enabled, ntfy_server, ntfy_topic,
            notify_start, notify_1h, notify_25m, notify_crossed,
            lockdown_enabled, lockdown_secret, timezone,
            adguard_enabled, adguard_url, adguard_username, adguard_password,
            adguard_client_id, adguard_services
        ) VALUES (?, 1, ?, ?, 1, 1, 1, 1, 1, ?, ?, 0, '', '', '', '', ?)
        "#,
    )
    .bind(uid)
    .bind(&server)
    .bind(&topic)
    .bind(&secret)
    .bind(&tz)
    .bind(&def_svc)
    .execute(pool)
    .await?;

    Ok(UserSettings {
        user_id: uid.to_string(),
        ntfy_enabled: true,
        ntfy_server: server,
        ntfy_topic: topic,
        notify_start: true,
        notify_1h: true,
        notify_25m: true,
        notify_crossed: true,
        lockdown_enabled: true,
        lockdown_secret: secret,
        locked_until: None,
        missed_deadlines_today: 0,
        timezone: tz,
        adguard_enabled: false,
        adguard_url: String::new(),
        adguard_username: String::new(),
        adguard_password: String::new(),
        adguard_client_id: String::new(),
        adguard_services: def_svc,
    })
}
