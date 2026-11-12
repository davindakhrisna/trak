#[cfg(feature = "ssr")]
use crate::backend::ntfy::send_ntfy_notification;
#[cfg(feature = "ssr")]
use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
#[cfg(feature = "ssr")]
use sqlx::{Pool, Row, Sqlite};

pub struct NotifyFlags {
    pub ntfy_on: bool,
    pub n_start: bool,
    pub n_1h: bool,
    pub n_25m: bool,
    pub n_cross: bool,
}

#[cfg(feature = "ssr")]
async fn log_sent(pool: &Pool<Sqlite>, uid: &str, tid: &str, ev: &str, date: &str) -> bool {
    let now = Utc::now().to_rfc3339();
    let id = uuid::Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO notification_log (id, user_id, task_id, event_type, task_date, sent_at) VALUES (?, ?, ?, ?, ?, ?)")
        .bind(&id).bind(uid).bind(tid).bind(ev).bind(date).bind(&now)
        .execute(pool).await.is_ok()
}

#[cfg(feature = "ssr")]
fn parse_time(
    s: Option<String>,
    today: NaiveDate,
    tz: chrono_tz::Tz,
) -> Option<DateTime<chrono_tz::Tz>> {
    s.and_then(|val| {
        NaiveTime::parse_from_str(val.trim(), "%H:%M")
            .ok()
            .and_then(|t| today.and_time(t).and_local_timezone(tz).single())
    })
}

#[cfg(feature = "ssr")]
pub async fn check_user_tasks(
    pool: &Pool<Sqlite>,
    uid: &str,
    server: &str,
    topic: &str,
    tz: chrono_tz::Tz,
    now_local: DateTime<chrono_tz::Tz>,
    flags: &NotifyFlags,
) -> i64 {
    let today_naive = now_local.date_naive();
    let today_str = now_local.format("%Y-%m-%d").to_string();

    let tasks = match sqlx::query("SELECT id, title, start_time, deadline, completed FROM today_tasks WHERE user_id = ? AND task_date = ?")
        .bind(uid).bind(&today_str).fetch_all(pool).await {
            Ok(t) => t,
            Err(_) => return 0,
        };

    let mut missed_count: i64 = 0;

    for t in &tasks {
        let (tid, title, completed): (String, String, i64) =
            (t.get("id"), t.get("title"), t.get("completed"));
        let (start_opt, dl_opt): (Option<String>, Option<String>) =
            (t.get("start_time"), t.get("deadline"));

        if let Some(start_dt) = parse_time(start_opt, today_naive, tz) {
            let diff = now_local.signed_duration_since(start_dt);
            if (0..=15).contains(&diff.num_minutes())
                && flags.ntfy_on
                && flags.n_start
                && log_sent(pool, uid, &tid, "start", &today_str).await
            {
                let _ = send_ntfy_notification(
                    server,
                    topic,
                    "Trak: Task Started",
                    &format!("Starting now: {}", title),
                    3,
                    &["hourglass"],
                )
                .await;
            }
        }

        if completed == 0 {
            if let Some(dl_dt) = parse_time(dl_opt, today_naive, tz) {
                let diff = dl_dt.signed_duration_since(now_local);
                if (26..=60).contains(&diff.num_minutes())
                    && flags.ntfy_on
                    && flags.n_1h
                    && log_sent(pool, uid, &tid, "1h", &today_str).await
                {
                    let _ = send_ntfy_notification(
                        server,
                        topic,
                        "Trak: 1 Hour Remaining",
                        &format!("1 hour left for: {}", title),
                        3,
                        &["warning"],
                    )
                    .await;
                }
                if (1..=25).contains(&diff.num_minutes())
                    && flags.ntfy_on
                    && flags.n_25m
                    && log_sent(pool, uid, &tid, "25m", &today_str).await
                {
                    let _ = send_ntfy_notification(
                        server,
                        topic,
                        "Trak: 25 Minutes Remaining",
                        &format!("Urgent: 25 min left: {}", title),
                        4,
                        &["alarm_clock"],
                    )
                    .await;
                }
                if now_local > dl_dt {
                    missed_count += 1;
                    if log_sent(pool, uid, &tid, "crossed", &today_str).await {
                        let msg = if missed_count >= 3 {
                            format!(
                                "3+ DEADLINES MISSED: '{}' failed. Locked today & tomorrow.",
                                title
                            )
                        } else {
                            format!(
                                "DEADLINE MISSED: '{}' failed. Locked for rest of today.",
                                title
                            )
                        };
                        if flags.ntfy_on && flags.n_cross {
                            let _ = send_ntfy_notification(
                                server,
                                topic,
                                "Trak: DEADLINE CROSSED",
                                &msg,
                                5,
                                &["skull", "lock"],
                            )
                            .await;
                        }
                    }
                }
            }
        }
    }

    missed_count
}
