use crate::domain::{UserAccount, UserRole};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthStatus {
    pub authenticated: bool,
    pub user: Option<UserAccount>,
    pub has_admin: bool,
}

#[server]
pub async fn check_auth() -> Result<AuthStatus, ServerFnError> {
    use crate::backend::auth::get_auth_claims_from_context;
    use crate::backend::AppState;
    use sqlx::Row;

    let state = expect_context::<AppState>();
    let user_count: i64 = sqlx::query("SELECT COUNT(*) as c FROM users")
        .fetch_one(&state.pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .get("c");

    let has_admin = user_count > 0;
    let claims = get_auth_claims_from_context(&state);

    if let Some(c) = claims {
        let user_row = sqlx::query(
            "SELECT id, username, role, is_first_admin FROM users WHERE id = ?",
        )
        .bind(&c.user_id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        if let Some(row) = user_row {
            let role_str: String = row.get("role");
            let role = if role_str == "ADMIN" {
                UserRole::Admin
            } else {
                UserRole::User
            };
            let is_first_admin: i64 = row.get("is_first_admin");

            return Ok(AuthStatus {
                authenticated: true,
                user: Some(UserAccount {
                    id: row.get("id"),
                    username: row.get("username"),
                    password: String::new(),
                    role,
                    is_first_admin: is_first_admin == 1,
                }),
                has_admin,
            });
        }
    }

    Ok(AuthStatus {
        authenticated: false,
        user: None,
        has_admin,
    })
}

#[server]
pub async fn register_first_admin(
    username: String,
    password: String,
) -> Result<UserAccount, ServerFnError> {
    use crate::backend::auth::{hash_password, set_session_cookie, sign_token, AuthClaims};
    use crate::backend::AppState;
    use chrono::Utc;
    use sqlx::Row;
    use uuid::Uuid;

    let state = expect_context::<AppState>();
    let count: i64 = sqlx::query("SELECT COUNT(*) as c FROM users")
        .fetch_one(&state.pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .get("c");

    if count > 0 {
        return Err(ServerFnError::new("Registration is closed. An admin already exists."));
    }

    let uname = username.trim().to_string();
    if uname.is_empty() || uname.len() > 32 {
        return Err(ServerFnError::new("Username must be between 1 and 32 characters."));
    }
    if password.is_empty() || password.len() > 128 {
        return Err(ServerFnError::new("Password must be between 1 and 128 characters."));
    }

    let pw_hash = hash_password(&password).map_err(ServerFnError::new)?;
    let user_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        r#"
        INSERT INTO users (id, username, password_hash, role, is_first_admin, timezone, created_at)
        VALUES (?, ?, ?, 'ADMIN', 1, 'UTC', ?)
        "#,
    )
    .bind(&user_id)
    .bind(&uname)
    .bind(&pw_hash)
    .bind(&now)
    .execute(&state.pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    let claims = AuthClaims::new(user_id.clone(), uname.clone(), UserRole::Admin);
    let token = sign_token(&claims, &state.secret).map_err(ServerFnError::new)?;
    set_session_cookie(&token);

    Ok(UserAccount {
        id: user_id,
        username: uname,
        password: String::new(),
        role: UserRole::Admin,
        is_first_admin: true,
    })
}

#[server]
pub async fn login(username: String, password: String) -> Result<UserAccount, ServerFnError> {
    use crate::backend::auth::{set_session_cookie, sign_token, verify_password, AuthClaims};
    use crate::backend::AppState;
    use sqlx::Row;

    let state = expect_context::<AppState>();
    let uname = username.trim().to_string();

    let user_row = sqlx::query(
        "SELECT id, username, password_hash, role, is_first_admin FROM users WHERE username = ?",
    )
    .bind(&uname)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    let row = match user_row {
        Some(r) => r,
        None => return Err(ServerFnError::new("Invalid username or password.")),
    };

    let pw_hash: String = row.get("password_hash");
    if !verify_password(&password, &pw_hash) {
        return Err(ServerFnError::new("Invalid username or password."));
    }

    let user_id: String = row.get("id");
    let role_str: String = row.get("role");
    let role = if role_str == "ADMIN" {
        UserRole::Admin
    } else {
        UserRole::User
    };
    let is_first_admin: i64 = row.get("is_first_admin");

    let claims = AuthClaims::new(user_id.clone(), uname.clone(), role);
    let token = sign_token(&claims, &state.secret).map_err(ServerFnError::new)?;
    set_session_cookie(&token);

    Ok(UserAccount {
        id: user_id,
        username: uname,
        password: String::new(),
        role,
        is_first_admin: is_first_admin == 1,
    })
}

#[server]
pub async fn logout() -> Result<(), ServerFnError> {
    use crate::backend::auth::clear_session_cookie;
    clear_session_cookie();
    Ok(())
}
