use crate::domain::{RecurringTemplate, Weekday};
use leptos::prelude::*;

#[cfg(feature = "ssr")]
fn weekday_from_int(val: i64) -> Weekday {
    match val {
        1 => Weekday::Mon,
        2 => Weekday::Tue,
        3 => Weekday::Wed,
        4 => Weekday::Thu,
        5 => Weekday::Fri,
        6 => Weekday::Sat,
        _ => Weekday::Sun,
    }
}

#[cfg(feature = "ssr")]
fn weekday_to_int(day: Weekday) -> i64 {
    match day {
        Weekday::Mon => 1,
        Weekday::Tue => 2,
        Weekday::Wed => 3,
        Weekday::Thu => 4,
        Weekday::Fri => 5,
        Weekday::Sat => 6,
        Weekday::Sun => 7,
    }
}

#[server]
pub async fn fetch_templates() -> Result<Vec<RecurringTemplate>, ServerFnError> {
    use crate::backend::auth::get_auth_claims_from_context;
    use crate::backend::AppState;
    use sqlx::Row;

    let state = expect_context::<AppState>();
    let claims =
        get_auth_claims_from_context(&state).ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    let rows = sqlx::query(
        r#"
        SELECT id, day_of_week, title, start_time, deadline
        FROM recurring_templates
        WHERE user_id = ?
        ORDER BY day_of_week ASC, created_at ASC
        "#,
    )
    .bind(&claims.user_id)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    let list = rows
        .into_iter()
        .map(|r| {
            let day_int: i64 = r.get("day_of_week");
            RecurringTemplate {
                id: r.get("id"),
                day: weekday_from_int(day_int),
                title: r.get("title"),
                start: r.get("start_time"),
                deadline: r.get("deadline"),
            }
        })
        .collect();

    Ok(list)
}

#[server]
pub async fn create_template(
    day: Weekday,
    title: String,
    start: Option<String>,
    deadline: Option<String>,
) -> Result<RecurringTemplate, ServerFnError> {
    use crate::backend::auth::get_auth_claims_from_context;
    use crate::backend::AppState;
    use chrono::Utc;
    use uuid::Uuid;

    let state = expect_context::<AppState>();
    let claims =
        get_auth_claims_from_context(&state).ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    let mut clean_title = title.replace(['\r', '\n'], " ").trim().to_string();
    if clean_title.is_empty() {
        return Err(ServerFnError::new(
            "Template task description cannot be empty.",
        ));
    }
    if clean_title.len() > 200 {
        clean_title.truncate(200);
    }

    let clean_start = match start.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
        Some(s) => {
            chrono::NaiveTime::parse_from_str(s, "%H:%M")
                .map_err(|_| ServerFnError::new("Start time must be HH:MM format (e.g. 09:00)."))?;
            Some(s.to_string())
        }
        None => None,
    };

    let clean_deadline = match deadline.as_deref().map(str::trim).filter(|d| !d.is_empty()) {
        Some(d) => {
            chrono::NaiveTime::parse_from_str(d, "%H:%M")
                .map_err(|_| ServerFnError::new("Deadline must be HH:MM format (e.g. 17:30)."))?;
            Some(d.to_string())
        }
        None => None,
    };

    let tpl_id = Uuid::new_v4().to_string();
    let day_int = weekday_to_int(day);
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        r#"
        INSERT INTO recurring_templates (id, user_id, day_of_week, title, start_time, deadline, created_at)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&tpl_id)
    .bind(&claims.user_id)
    .bind(day_int)
    .bind(&clean_title)
    .bind(&clean_start)
    .bind(&clean_deadline)
    .bind(&now)
    .execute(&state.pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(RecurringTemplate {
        id: tpl_id,
        day,
        title: clean_title,
        start: clean_start,
        deadline: clean_deadline,
    })
}

#[server]
pub async fn delete_template(template_id: String) -> Result<(), ServerFnError> {
    use crate::backend::auth::get_auth_claims_from_context;
    use crate::backend::AppState;

    let state = expect_context::<AppState>();
    let claims =
        get_auth_claims_from_context(&state).ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    sqlx::query("DELETE FROM recurring_templates WHERE id = ? AND user_id = ?")
        .bind(&template_id)
        .bind(&claims.user_id)
        .execute(&state.pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}
