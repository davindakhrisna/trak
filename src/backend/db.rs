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
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
