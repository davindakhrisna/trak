#[cfg(feature = "ssr")]
use chrono::{Datelike, NaiveDate, Utc};
#[cfg(feature = "ssr")]
use sqlx::{Pool, Row, Sqlite};
#[cfg(feature = "ssr")]
use uuid::Uuid;

#[cfg(feature = "ssr")]
pub async fn provision_for_user(
    pool: &Pool<Sqlite>,
    user_id: &str,
    target_date: NaiveDate,
) -> Result<usize, sqlx::Error> {
    let day_of_week = target_date.weekday().number_from_monday() as i64;
    let date_str = target_date.format("%Y-%m-%d").to_string();

    let templates = sqlx::query(
        r#"
        SELECT id, title, start_time, deadline
        FROM recurring_templates
        WHERE user_id = ? AND day_of_week = ?
        "#,
    )
    .bind(user_id)
    .bind(day_of_week)
    .fetch_all(pool)
    .await?;

    let mut count = 0;
    for tpl in templates {
        let tpl_id: String = tpl.get("id");
        let title: String = tpl.get("title");
        let start_time: Option<String> = tpl.get("start_time");
        let deadline: Option<String> = tpl.get("deadline");

        let mut tx = pool.begin().await?;
        let log_id = Uuid::new_v4().to_string();
        let log_res = sqlx::query(
            r#"
            INSERT OR IGNORE INTO provision_log (id, user_id, template_id, provision_date)
            VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(&log_id)
        .bind(user_id)
        .bind(&tpl_id)
        .bind(&date_str)
        .execute(&mut *tx)
        .await?;

        if log_res.rows_affected() > 0 {
            let task_id = Uuid::new_v4().to_string();
            let now = Utc::now().to_rfc3339();
            sqlx::query(
                r#"
                INSERT INTO today_tasks (
                    id, user_id, title, start_time, deadline,
                    completed, from_template, task_date, created_at
                ) VALUES (?, ?, ?, ?, ?, 0, 1, ?, ?)
                "#,
            )
            .bind(&task_id)
            .bind(user_id)
            .bind(&title)
            .bind(&start_time)
            .bind(&deadline)
            .bind(&date_str)
            .bind(&now)
            .execute(&mut *tx)
            .await?;

            count += 1;
        }
        tx.commit().await?;
    }

    Ok(count)
}

#[cfg(feature = "ssr")]
pub async fn provision_all_users(pool: &Pool<Sqlite>) -> Result<usize, sqlx::Error> {
    let users = sqlx::query("SELECT id FROM users").fetch_all(pool).await?;

    let today = Utc::now().date_naive();
    let mut total = 0;
    for u in users {
        let uid: String = u.get("id");
        if let Ok(c) = provision_for_user(pool, &uid, today).await {
            total += c;
        }
    }

    Ok(total)
}

#[cfg(feature = "ssr")]
pub fn spawn_midnight_ticker(pool: Pool<Sqlite>) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
        let mut last_date = Utc::now().date_naive();

        loop {
            interval.tick().await;
            let current_date = Utc::now().date_naive();
            if current_date != last_date {
                last_date = current_date;
                let _ = provision_all_users(&pool).await;
            }
        }
    });
}
