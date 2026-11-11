use crate::domain::{RecurringTemplate, TodayTask, UserAccount, UserRole, Weekday};

pub fn default_users() -> Vec<UserAccount> {
    vec![
        UserAccount {
            id: 1,
            username: "admin".to_string(),
            password: "password".to_string(),
            role: UserRole::Admin,
            is_first_admin: true,
        },
        UserAccount {
            id: 2,
            username: "alexalexalexalexalexalexalex".to_string(),
            password: "password".to_string(),
            role: UserRole::Admin,
            is_first_admin: false,
        },
        UserAccount {
            id: 3,
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
            id: 101,
            day: today,
            title: "Audit overnight backup logs".to_string(),
        },
        RecurringTemplate {
            id: 102,
            day: today,
            title: "Clear inbox to zero before noon".to_string(),
        },
        RecurringTemplate {
            id: 103,
            day: Weekday::Mon,
            title: "Weekly sprint scope alignment".to_string(),
        },
        RecurringTemplate {
            id: 104,
            day: Weekday::Fri,
            title: "Deploy release candidate & tag changelog".to_string(),
        },
        RecurringTemplate {
            id: 105,
            day: Weekday::Sun,
            title: "Reset workspace for upcoming week".to_string(),
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
            id: (idx as u64) + 1,
            title: tpl.title.clone(),
            completed: false,
            from_template: true,
        })
        .collect();

    let extra = [
        ("Review pull requests for core router", true),
        ("Sync with infrastructure team", false),
        ("Profile memory allocations on WASM bundle", false),
        ("Refactor database migration runners", false),
        ("Run end-to-end regression benchmarks", false),
    ];
    for (i, (title, done)) in extra.iter().enumerate() {
        list.push(TodayTask {
            id: 50 + (i as u64),
            title: title.to_string(),
            completed: *done,
            from_template: false,
        });
    }
    list
}
