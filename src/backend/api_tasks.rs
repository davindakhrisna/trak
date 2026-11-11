use crate::domain::TodayTask;
use leptos::prelude::*;

#[server]
pub async fn fetch_today_tasks(
    client_date: Option<String>,
) -> Result<Vec<TodayTask>, ServerFnError> {
    use crate::backend::auth::get_auth_claims_from_context;
    use crate::backend::provision::provision_for_user;
    use crate::backend::AppState;
    use chrono::{NaiveDate, Utc};
    use sqlx::Row;

    let state = expect_context::<AppState>();
    let claims = get_auth_claims_from_context(&state)
        .ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    let today = client_date
        .as_deref()
        .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
        .unwrap_or_else(|| Utc::now().date_naive());

    let _ = provision_for_user(&state.pool, &claims.user_id, today).await;

    let date_str = today.format("%Y-%m-%d").to_string();
    let rows = sqlx::query(
        r#"
        SELECT id, title, start_time, deadline, completed, from_template
        FROM today_tasks
        WHERE user_id = ? AND task_date = ?
        ORDER BY created_at ASC
        "#,
    )
    .bind(&claims.user_id)
    .bind(&date_str)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    let tasks = rows
        .into_iter()
        .map(|r| {
            let completed_int: i64 = r.get("completed");
            let from_tpl_int: i64 = r.get("from_template");
            TodayTask {
                id: r.get("id"),
                title: r.get("title"),
                start: r.get("start_time"),
                deadline: r.get("deadline"),
                completed: completed_int == 1,
                from_template: from_tpl_int == 1,
            }
        })
        .collect();

    Ok(tasks)
}

#[server]
pub async fn create_today_task(
    title: String,
    start: Option<String>,
    deadline: Option<String>,
    client_date: Option<String>,
) -> Result<TodayTask, ServerFnError> {
    use crate::backend::auth::get_auth_claims_from_context;
    use crate::backend::AppState;
    use chrono::{NaiveDate, Utc};
    use uuid::Uuid;

    let state = expect_context::<AppState>();
    let claims = get_auth_claims_from_context(&state)
        .ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    let mut clean_title = title.trim().to_string();
    if clean_title.is_empty() {
        return Err(ServerFnError::new("Task title cannot be empty."));
    }
    if clean_title.len() > 200 {
        clean_title.truncate(200);
    }

    let clean_start = start
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .map(|mut s| {
            if s.len() > 30 {
                s.truncate(30);
            }
            s
        });

    let clean_deadline = deadline
        .map(|d| d.trim().to_string())
        .filter(|d| !d.is_empty())
        .map(|mut d| {
            if d.len() > 30 {
                d.truncate(30);
            }
            d
        });

    let today = client_date
        .as_deref()
        .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
        .unwrap_or_else(|| Utc::now().date_naive());
    let date_str = today.format("%Y-%m-%d").to_string();

    let task_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        r#"
        INSERT INTO today_tasks (
            id, user_id, title, start_time, deadline,
            completed, from_template, task_date, created_at
        ) VALUES (?, ?, ?, ?, ?, 0, 0, ?, ?)
        "#,
    )
    .bind(&task_id)
    .bind(&claims.user_id)
    .bind(&clean_title)
    .bind(&clean_start)
    .bind(&clean_deadline)
    .bind(&date_str)
    .bind(&now)
    .execute(&state.pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(TodayTask {
        id: task_id,
        title: clean_title,
        start: clean_start,
        deadline: clean_deadline,
        completed: false,
        from_template: false,
    })
}

#[server]
pub async fn toggle_task_completed(task_id: String) -> Result<bool, ServerFnError> {
    use crate::backend::auth::get_auth_claims_from_context;
    use crate::backend::AppState;
    use sqlx::Row;

    let state = expect_context::<AppState>();
    let claims = get_auth_claims_from_context(&state)
        .ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    let row = sqlx::query("SELECT completed FROM today_tasks WHERE id = ? AND user_id = ?")
        .bind(&task_id)
        .bind(&claims.user_id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let current_completed: i64 = match row {
        Some(r) => r.get("completed"),
        None => return Err(ServerFnError::new("Task not found.")),
    };

    let next_completed = if current_completed == 1 { 0 } else { 1 };
    sqlx::query("UPDATE today_tasks SET completed = ? WHERE id = ? AND user_id = ?")
        .bind(next_completed)
        .bind(&task_id)
        .bind(&claims.user_id)
        .execute(&state.pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(next_completed == 1)
}

#[server]
pub async fn delete_today_task(task_id: String) -> Result<(), ServerFnError> {
    use crate::backend::auth::get_auth_claims_from_context;
    use crate::backend::AppState;

    let state = expect_context::<AppState>();
    let claims = get_auth_claims_from_context(&state)
        .ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    sqlx::query("DELETE FROM today_tasks WHERE id = ? AND user_id = ?")
        .bind(&task_id)
        .bind(&claims.user_id)
        .execute(&state.pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}
