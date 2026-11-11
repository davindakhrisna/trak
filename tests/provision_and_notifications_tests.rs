#[cfg(feature = "ssr")]
mod tests {
    use chrono::{NaiveDate, NaiveTime, TimeZone};
    use sqlx::Row;
    use trak::backend::db::init_db;
    use trak::backend::notification_task_checker::{check_user_tasks, NotifyFlags};
    use trak::backend::provision::provision_for_user;

    #[tokio::test]
    async fn test_recurring_template_provisioning_and_idempotency() {
        let pool = init_db("sqlite::memory:").await.unwrap();

        // 1. Create user
        sqlx::query("INSERT INTO users (id, username, password_hash, role, is_first_admin, timezone, created_at) VALUES ('u1', 'alice', 'h', 'USER', 0, 'UTC', '2026-01-01T00:00:00Z')")
            .execute(&pool)
            .await
            .unwrap();

        // 2. Insert recurring template for Monday (day_of_week = 1)
        sqlx::query("INSERT INTO recurring_templates (id, user_id, title, day_of_week, start_time, deadline, created_at) VALUES ('tpl-1', 'u1', 'Monday Standup', 1, '09:00', '10:00', '2026-01-01T00:00:00Z')")
            .execute(&pool)
            .await
            .unwrap();

        // Target a Monday: 2026-11-09 was a Monday
        let monday = NaiveDate::from_ymd_opt(2026, 11, 9).unwrap();
        let tuesday = NaiveDate::from_ymd_opt(2026, 11, 10).unwrap();

        // Provision for Monday -> should create 1 task
        let created = provision_for_user(&pool, "u1", monday).await.unwrap();
        assert_eq!(created, 1, "Monday template must generate 1 today task");

        // Verify task in database
        let row = sqlx::query("SELECT title, from_template, task_date FROM today_tasks WHERE user_id = 'u1' AND task_date = '2026-11-09'")
            .fetch_one(&pool)
            .await
            .unwrap();
        let title: String = row.get("title");
        let from_template: i64 = row.get("from_template");
        assert_eq!(title, "Monday Standup");
        assert_eq!(from_template, 1);

        // Provision for Monday a second time -> idempotency check (must NOT duplicate)
        let duplicate = provision_for_user(&pool, "u1", monday).await.unwrap();
        assert_eq!(
            duplicate, 0,
            "Idempotent: duplicate provision must produce 0 tasks"
        );

        // Provision for Tuesday -> should produce 0 tasks (no Tuesday template)
        let tuesday_tasks = provision_for_user(&pool, "u1", tuesday).await.unwrap();
        assert_eq!(
            tuesday_tasks, 0,
            "Monday template must not provision on Tuesday"
        );
    }

    #[tokio::test]
    async fn test_notification_task_checker_missed_deadlines_and_deduplication() {
        let pool = init_db("sqlite::memory:").await.unwrap();
        let tz = chrono_tz::UTC;

        // Create user
        sqlx::query("INSERT INTO users (id, username, password_hash, role, is_first_admin, timezone, created_at) VALUES ('u2', 'bob', 'h', 'USER', 0, 'UTC', '2026-01-01T00:00:00Z')")
            .execute(&pool)
            .await
            .unwrap();

        let today_date = NaiveDate::from_ymd_opt(2026, 11, 11).unwrap();
        let today_str = "2026-11-11";

        // Current simulated time: 14:00 UTC
        let now_local =
            tz.from_utc_datetime(&today_date.and_time(NaiveTime::from_hms_opt(14, 0, 0).unwrap()));

        // Task 1: Completed task whose deadline was 12:00 (Should NOT count as missed!)
        sqlx::query("INSERT INTO today_tasks (id, user_id, title, start_time, deadline, completed, from_template, task_date, created_at) VALUES ('t1', 'u2', 'Done Task', '11:00', '12:00', 1, 0, ?, '2026-01-01T00:00:00Z')")
            .bind(today_str)
            .execute(&pool)
            .await
            .unwrap();

        // Task 2: Uncompleted task whose deadline was 13:30 (Missed 30 mins ago -> MISSED)
        sqlx::query("INSERT INTO today_tasks (id, user_id, title, start_time, deadline, completed, from_template, task_date, created_at) VALUES ('t2', 'u2', 'Missed Task 1', '12:00', '13:30', 0, 0, ?, '2026-01-01T00:00:00Z')")
            .bind(today_str)
            .execute(&pool)
            .await
            .unwrap();

        // Task 3: Uncompleted task whose deadline was 13:45 (Missed 15 mins ago -> MISSED)
        sqlx::query("INSERT INTO today_tasks (id, user_id, title, start_time, deadline, completed, from_template, task_date, created_at) VALUES ('t3', 'u2', 'Missed Task 2', '12:00', '13:45', 0, 0, ?, '2026-01-01T00:00:00Z')")
            .bind(today_str)
            .execute(&pool)
            .await
            .unwrap();

        let flags = NotifyFlags {
            ntfy_on: true,
            n_start: true,
            n_1h: true,
            n_25m: true,
            n_cross: true,
        };

        // First check: should identify exactly 2 missed tasks
        let missed_count = check_user_tasks(
            &pool,
            "u2",
            "https://ntfy.sh",
            "test-topic",
            tz,
            now_local,
            &flags,
        )
        .await;
        assert_eq!(
            missed_count, 2,
            "Only uncompleted tasks past deadline count as missed"
        );

        // Verify notification_log has 2 crossed events
        let log_count: i64 = sqlx::query("SELECT COUNT(*) as c FROM notification_log WHERE user_id = 'u2' AND event_type = 'crossed'")
            .fetch_one(&pool)
            .await
            .unwrap()
            .get("c");
        assert_eq!(log_count, 2, "Crossed events logged for both missed tasks");

        // Second check: log_sent will return false (already logged), preventing duplicate notifications
        let second_check_missed = check_user_tasks(
            &pool,
            "u2",
            "https://ntfy.sh",
            "test-topic",
            tz,
            now_local,
            &flags,
        )
        .await;
        assert_eq!(
            second_check_missed, 2,
            "Missed count calculation remains consistent"
        );

        // notification_log count must still be 2 (no duplicate inserts)
        let log_count2: i64 = sqlx::query("SELECT COUNT(*) as c FROM notification_log WHERE user_id = 'u2' AND event_type = 'crossed'")
            .fetch_one(&pool)
            .await
            .unwrap()
            .get("c");
        assert_eq!(log_count2, 2, "No duplicate notification logs recorded");
    }

    #[tokio::test]
    async fn test_notification_task_checker_1h_and_25m_windows() {
        let pool = init_db("sqlite::memory:").await.unwrap();
        let tz = chrono_tz::UTC;
        let today_date = NaiveDate::from_ymd_opt(2026, 11, 11).unwrap();
        let today_str = "2026-11-11";

        sqlx::query("INSERT INTO users (id, username, password_hash, role, is_first_admin, timezone, created_at) VALUES ('u3', 'charlie', 'h', 'USER', 0, 'UTC', '2026-01-01T00:00:00Z')")
            .execute(&pool)
            .await
            .unwrap();

        // Simulated time: 10:00 UTC
        let now_local =
            tz.from_utc_datetime(&today_date.and_time(NaiveTime::from_hms_opt(10, 0, 0).unwrap()));

        // Task A: Deadline at 10:45 (45 mins away -> inside 26..=60 min 1-hour alert window)
        sqlx::query("INSERT INTO today_tasks (id, user_id, title, start_time, deadline, completed, from_template, task_date, created_at) VALUES ('ta', 'u3', '1h Warning Task', '09:00', '10:45', 0, 0, ?, '2026-01-01T00:00:00Z')")
            .bind(today_str)
            .execute(&pool)
            .await
            .unwrap();

        // Task B: Deadline at 10:15 (15 mins away -> inside 1..=25 min urgent alert window)
        sqlx::query("INSERT INTO today_tasks (id, user_id, title, start_time, deadline, completed, from_template, task_date, created_at) VALUES ('tb', 'u3', '25m Warning Task', '09:30', '10:15', 0, 0, ?, '2026-01-01T00:00:00Z')")
            .bind(today_str)
            .execute(&pool)
            .await
            .unwrap();

        let flags = NotifyFlags {
            ntfy_on: true,
            n_start: false,
            n_1h: true,
            n_25m: true,
            n_cross: false,
        };

        check_user_tasks(
            &pool,
            "u3",
            "https://ntfy.sh",
            "test-topic",
            tz,
            now_local,
            &flags,
        )
        .await;

        let has_1h: bool = sqlx::query("SELECT 1 FROM notification_log WHERE user_id = 'u3' AND task_id = 'ta' AND event_type = '1h'")
            .fetch_optional(&pool)
            .await
            .unwrap()
            .is_some();
        assert!(
            has_1h,
            "1h warning event must be logged for task with 45m left"
        );

        let has_25m: bool = sqlx::query("SELECT 1 FROM notification_log WHERE user_id = 'u3' AND task_id = 'tb' AND event_type = '25m'")
            .fetch_optional(&pool)
            .await
            .unwrap()
            .is_some();
        assert!(
            has_25m,
            "25m warning event must be logged for task with 15m left"
        );
    }
}
