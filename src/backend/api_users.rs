use crate::domain::{UserAccount, UserRole};
use leptos::prelude::*;

#[server]
pub async fn fetch_users_list() -> Result<Vec<UserAccount>, ServerFnError> {
    use crate::backend::auth::get_auth_claims_from_context;
    use crate::backend::AppState;
    use sqlx::Row;

    let state = expect_context::<AppState>();
    let claims =
        get_auth_claims_from_context(&state).ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    if !claims.is_admin() {
        return Err(ServerFnError::new("Forbidden: Admin privileges required."));
    }

    let rows = sqlx::query(
        r#"
        SELECT id, username, role, is_first_admin
        FROM users
        ORDER BY is_first_admin DESC, created_at ASC
        "#,
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    let list = rows
        .into_iter()
        .map(|r| {
            let role_str: String = r.get("role");
            let role = if role_str == "ADMIN" {
                UserRole::Admin
            } else {
                UserRole::User
            };
            let is_first: i64 = r.get("is_first_admin");
            UserAccount {
                id: r.get("id"),
                username: r.get("username"),
                password: String::new(),
                role,
                is_first_admin: is_first == 1,
            }
        })
        .collect();

    Ok(list)
}

#[server]
pub async fn admin_create_user(
    username: String,
    password: String,
    as_admin: bool,
) -> Result<UserAccount, ServerFnError> {
    use crate::backend::auth::{get_auth_claims_from_context, hash_password};
    use crate::backend::AppState;
    use chrono::Utc;
    use sqlx::Row;
    use uuid::Uuid;

    let state = expect_context::<AppState>();
    let claims =
        get_auth_claims_from_context(&state).ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    if !claims.is_admin() {
        return Err(ServerFnError::new("Forbidden: Admin privileges required."));
    }

    let clean_user = username.trim().to_string();
    if clean_user.len() < 3 || clean_user.len() > 32 {
        return Err(ServerFnError::new(
            "Username must be between 3 and 32 characters.",
        ));
    }
    if password.len() < 4 || password.len() > 128 {
        return Err(ServerFnError::new(
            "Password must be between 4 and 128 characters.",
        ));
    }

    let exists: i64 =
        sqlx::query("SELECT COUNT(*) as c FROM users WHERE LOWER(username) = LOWER(?)")
            .bind(&clean_user)
            .fetch_one(&state.pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?
            .get("c");

    if exists > 0 {
        return Err(ServerFnError::new(
            "A user with that username already exists.",
        ));
    }

    let pw_hash = hash_password(&password).map_err(ServerFnError::new)?;
    let role = if as_admin {
        UserRole::Admin
    } else {
        UserRole::User
    };
    let new_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        r#"
        INSERT INTO users (id, username, password_hash, role, is_first_admin, timezone, created_at)
        VALUES (?, ?, ?, ?, 0, 'UTC', ?)
        "#,
    )
    .bind(&new_id)
    .bind(&clean_user)
    .bind(&pw_hash)
    .bind(role.label())
    .bind(&now)
    .execute(&state.pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(UserAccount {
        id: new_id,
        username: clean_user,
        password: String::new(),
        role,
        is_first_admin: false,
    })
}

#[server]
pub async fn admin_toggle_user_role(target_id: String) -> Result<UserRole, ServerFnError> {
    use crate::backend::auth::get_auth_claims_from_context;
    use crate::backend::AppState;
    use sqlx::Row;

    let state = expect_context::<AppState>();
    let claims =
        get_auth_claims_from_context(&state).ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    if !claims.is_admin() {
        return Err(ServerFnError::new("Forbidden: Admin privileges required."));
    }

    let row = sqlx::query("SELECT role, is_first_admin FROM users WHERE id = ?")
        .bind(&target_id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let r = row.ok_or_else(|| ServerFnError::new("User not found."))?;
    let is_first: i64 = r.get("is_first_admin");
    if is_first == 1 {
        return Err(ServerFnError::new(
            "Cannot modify the root administrator's role.",
        ));
    }

    let current_role: String = r.get("role");
    let next_role = if current_role == "ADMIN" {
        UserRole::User
    } else {
        UserRole::Admin
    };

    sqlx::query("UPDATE users SET role = ? WHERE id = ?")
        .bind(next_role.label())
        .bind(&target_id)
        .execute(&state.pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(next_role)
}

#[server]
pub async fn admin_delete_user(target_id: String) -> Result<(), ServerFnError> {
    use crate::backend::auth::get_auth_claims_from_context;
    use crate::backend::AppState;
    use sqlx::Row;

    let state = expect_context::<AppState>();
    let claims =
        get_auth_claims_from_context(&state).ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    if !claims.is_admin() {
        return Err(ServerFnError::new("Forbidden: Admin privileges required."));
    }

    if target_id == claims.user_id {
        return Err(ServerFnError::new("Cannot delete your own active account."));
    }

    let row = sqlx::query("SELECT is_first_admin FROM users WHERE id = ?")
        .bind(&target_id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let r = row.ok_or_else(|| ServerFnError::new("User not found."))?;
    let is_first: i64 = r.get("is_first_admin");
    if is_first == 1 {
        return Err(ServerFnError::new("Cannot delete the root administrator."));
    }

    sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(&target_id)
        .execute(&state.pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}
