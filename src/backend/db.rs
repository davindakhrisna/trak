#[cfg(feature = "ssr")]
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
#[cfg(feature = "ssr")]
use sqlx::{Pool, Sqlite};
#[cfg(feature = "ssr")]
use std::str::FromStr;

#[cfg(feature = "ssr")]
pub async fn init_db(database_url: &str) -> Result<Pool<Sqlite>, sqlx::Error> {
    let options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .busy_timeout(std::time::Duration::from_secs(5))
        .foreign_keys(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    migrate(&pool).await?;
    Ok(pool)
}

#[cfg(feature = "ssr")]
async fn migrate(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            role TEXT NOT NULL,
            is_first_admin INTEGER NOT NULL DEFAULT 0,
            timezone TEXT NOT NULL DEFAULT 'UTC',
            created_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS today_tasks (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            title TEXT NOT NULL,
            start_time TEXT,
            deadline TEXT,
            completed INTEGER NOT NULL DEFAULT 0,
            from_template INTEGER NOT NULL DEFAULT 0,
            task_date TEXT NOT NULL,
            created_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS recurring_templates (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            day_of_week INTEGER NOT NULL,
            title TEXT NOT NULL,
            start_time TEXT,
            deadline TEXT,
            created_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS provision_log (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            template_id TEXT NOT NULL REFERENCES recurring_templates(id) ON DELETE CASCADE,
            provision_date TEXT NOT NULL,
            UNIQUE(user_id, template_id, provision_date)
        );

        CREATE INDEX IF NOT EXISTS idx_tasks_user_date ON today_tasks(user_id, task_date);
        CREATE INDEX IF NOT EXISTS idx_templates_user_day ON recurring_templates(user_id, day_of_week);

        CREATE TABLE IF NOT EXISTS user_settings (
            user_id TEXT PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
            ntfy_enabled INTEGER NOT NULL DEFAULT 1,
            ntfy_server TEXT NOT NULL DEFAULT 'https://ntfy.sh',
            ntfy_topic TEXT NOT NULL,
            notify_start INTEGER NOT NULL DEFAULT 1,
            notify_1h INTEGER NOT NULL DEFAULT 1,
            notify_25m INTEGER NOT NULL DEFAULT 1,
            notify_crossed INTEGER NOT NULL DEFAULT 1,
            lockdown_enabled INTEGER NOT NULL DEFAULT 1,
            lockdown_secret TEXT UNIQUE NOT NULL,
            locked_until TEXT,
            timezone TEXT NOT NULL DEFAULT 'UTC'
        );

        CREATE TABLE IF NOT EXISTS notification_log (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            task_id TEXT NOT NULL REFERENCES today_tasks(id) ON DELETE CASCADE,
            event_type TEXT NOT NULL,
            task_date TEXT NOT NULL,
            sent_at TEXT NOT NULL,
            UNIQUE(task_id, event_type, task_date)
        );

        CREATE INDEX IF NOT EXISTS idx_notif_log_user ON notification_log(user_id, task_date);
        CREATE INDEX IF NOT EXISTS idx_user_settings_secret ON user_settings(lockdown_secret);
        CREATE UNIQUE INDEX IF NOT EXISTS idx_single_first_admin ON users(is_first_admin) WHERE is_first_admin = 1;
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
