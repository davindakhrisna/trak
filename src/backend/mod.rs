pub mod api_auth;
pub mod api_tasks;
pub mod api_templates;
pub mod api_users;
pub mod auth;
pub mod db;
pub mod provision;

#[cfg(feature = "ssr")]
use sqlx::{Pool, Sqlite};

#[cfg(feature = "ssr")]
#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Sqlite>,
    pub secret: Vec<u8>,
    pub leptos_options: leptos::config::LeptosOptions,
}

#[cfg(feature = "ssr")]
impl axum::extract::FromRef<AppState> for leptos::config::LeptosOptions {
    fn from_ref(state: &AppState) -> Self {
        state.leptos_options.clone()
    }
}
