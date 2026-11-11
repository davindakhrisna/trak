use crate::domain::{RecurringTemplate, TodayTask, UserAccount, UserRole, Weekday};

pub fn default_users() -> Vec<UserAccount> {
    vec![
        UserAccount {
            id: "1".to_string(),
            username: "admin".to_string(),
            password: "password".to_string(),
            role: UserRole::Admin,
            is_first_admin: true,
        },
        UserAccount {
            id: "2".to_string(),
            username: "alexalexalexalexalexalexalex".to_string(),
            password: "password".to_string(),
            role: UserRole::Admin,
            is_first_admin: false,
        },
        UserAccount {
            id: "3".to_string(),
            username: "marcus_aurelius_operations".to_string(),
            password: "password".to_string(),
            role: UserRole::User,
            is_first_admin: false,
        },
    ]
}

pub fn default_templates() -> Vec<RecurringTemplate> {
    let today = Weekday::today();
    let mut items = vec![
        RecurringTemplate {
            id: "101".to_string(),
            day: today,
            title: "Audit overnight backup logs".to_string(),
            start: Some("07:00".to_string()),
            deadline: Some("08:00".to_string()),
        },
        RecurringTemplate {
            id: "102".to_string(),
            day: today,
            title: "Clear inbox to zero before noon".to_string(),
            start: Some("11:00".to_string()),
            deadline: Some("12:00".to_string()),
        },
        RecurringTemplate {
            id: "103".to_string(),
            day: Weekday::Mon,
            title: "Weekly sprint scope alignment".to_string(),
            start: Some("09:30".to_string()),
            deadline: Some("10:30".to_string()),
        },
        RecurringTemplate {
            id: "104".to_string(),
            day: Weekday::Fri,
            title: "Deploy release candidate & tag changelog".to_string(),
            start: Some("15:00".to_string()),
            deadline: Some("16:00".to_string()),
        },
        RecurringTemplate {
            id: "105".to_string(),
            day: Weekday::Sun,
            title: "Reset workspace for upcoming week".to_string(),
            start: Some("19:00".to_string()),
            deadline: Some("20:00".to_string()),
        },
    ];
    items.dedup_by(|a, b| a.day == b.day && a.title == b.title);
    items
}

pub fn default_today_tasks(templates: &[RecurringTemplate]) -> Vec<TodayTask> {
    let today = Weekday::today();
    let mut list: Vec<TodayTask> = templates
        .iter()
        .filter(|tpl| tpl.day == today)
        .enumerate()
        .map(|(idx, tpl)| TodayTask {
            id: format!("task-{}", idx + 1),
            title: tpl.title.clone(),
            start: tpl.start.clone(),
            deadline: tpl.deadline.clone(),
            completed: false,
            from_template: true,
        })
        .collect();

    let extra = [
        (
            "Review pull requests for core router",
            true,
            Some("10:00"),
            Some("11:00"),
        ),
        (
            "Sync with infrastructure team",
            false,
            Some("14:00"),
            Some("14:30"),
        ),
        (
            "Profile memory allocations on WASM bundle",
            false,
            Some("15:00"),
            Some("17:00"),
        ),
        ("Refactor database migration runners", false, None, None),
        (
            "Run end-to-end regression benchmarks",
            false,
            Some("18:00"),
            Some("19:00"),
        ),
    ];
    for (i, (title, done, start, deadline)) in extra.iter().enumerate() {
        list.push(TodayTask {
            id: format!("task-{}", 50 + i),
            title: title.to_string(),
            start: start.map(|s| s.to_string()),
            deadline: deadline.map(|d| d.to_string()),
            completed: *done,
            from_template: false,
        });
    }
    list
}
