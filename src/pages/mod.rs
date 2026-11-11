pub mod auth;
pub mod register;
pub mod settings;
pub mod settings_guide;
pub mod settings_lockdown;
pub mod settings_ntfy;
pub mod templates;
pub mod today;
pub mod users;

pub use auth::LoginPage;
pub use register::RegisterPage;
pub use settings::SettingsPage;
pub use templates::TemplatesPage;
pub use today::TodayPage;
pub use users::UsersPage;
