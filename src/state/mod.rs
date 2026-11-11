pub mod auth_ops;
pub mod seed;

use crate::domain::{RecurringTemplate, ThemeMode, TodayTask, UserAccount, Weekday};
use leptos::prelude::*;

#[derive(Clone, Copy)]
pub struct AppStore {
    pub users: RwSignal<Vec<UserAccount>>,
    pub current_user_id: RwSignal<Option<String>>,
    pub first_register_claimed: RwSignal<bool>,
    pub auth_initialized: RwSignal<bool>,
    pub today_tasks: RwSignal<Vec<TodayTask>>,
    pub templates: RwSignal<Vec<RecurringTemplate>>,
    pub selected_day: RwSignal<Weekday>,
    pub theme_mode: RwSignal<ThemeMode>,
}

pub fn provide_app_store() -> AppStore {
    let store = AppStore {
        users: RwSignal::new(Vec::new()),
        current_user_id: RwSignal::new(None),
        first_register_claimed: RwSignal::new(false),
        auth_initialized: RwSignal::new(false),
        today_tasks: RwSignal::new(Vec::new()),
        templates: RwSignal::new(Vec::new()),
        selected_day: RwSignal::new(Weekday::today()),
        theme_mode: RwSignal::new(ThemeMode::System),
    };
    provide_context(store);
    store
}

pub fn use_app_store() -> AppStore {
    expect_context::<AppStore>()
}

fn clean_time_opt(val: Option<String>) -> Option<String> {
    val.map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .map(|mut s| {
            if s.len() > 30 {
                s.truncate(30);
            }
            s
        })
}

impl AppStore {
    fn alloc_id(&self) -> String {
        uuid::Uuid::new_v4().to_string()
    }

    pub fn current_user(&self) -> Option<UserAccount> {
        let uid = self.current_user_id.get()?;
        self.users.get().into_iter().find(|u| u.id == uid)
    }

    pub fn is_admin(&self) -> bool {
        self.current_user()
            .map(|u| u.role.is_admin())
            .unwrap_or(false)
    }

    pub fn add_today_task(&self, title: String, start: Option<String>, deadline: Option<String>) {
        let mut trimmed = title.trim().to_string();
        if trimmed.is_empty() {
            return;
        }
        if trimmed.len() > 200 {
            trimmed.truncate(200);
        }
        let clean_start = clean_time_opt(start);
        let clean_deadline = clean_time_opt(deadline);
        let id = self.alloc_id();
        self.today_tasks.update(|list| {
            list.insert(
                0,
                TodayTask {
                    id,
                    title: trimmed,
                    start: clean_start,
                    deadline: clean_deadline,
                    completed: false,
                    from_template: false,
                },
            );
        });
    }

    pub fn toggle_today_task(&self, id: &str) {
        self.today_tasks.update(|list| {
            if let Some(item) = list.iter_mut().find(|t| t.id == id) {
                item.completed = !item.completed;
            }
        });
    }

    pub fn remove_today_task(&self, id: &str) {
        self.today_tasks.update(|list| list.retain(|t| t.id != id));
    }

    pub fn add_template(
        &self,
        day: Weekday,
        title: String,
        start: Option<String>,
        deadline: Option<String>,
    ) {
        let mut trimmed = title.trim().to_string();
        if trimmed.is_empty() {
            return;
        }
        if trimmed.len() > 200 {
            trimmed.truncate(200);
        }
        let clean_start = clean_time_opt(start);
        let clean_deadline = clean_time_opt(deadline);
        let id = self.alloc_id();
        let cloned_start = clean_start.clone();
        let cloned_dl = clean_deadline.clone();
        self.templates.update(|list| {
            list.push(RecurringTemplate {
                id,
                day,
                title: trimmed.clone(),
                start: cloned_start,
                deadline: cloned_dl,
            });
        });
        if day == Weekday::today() {
            let task_id = self.alloc_id();
            self.today_tasks.update(|list| {
                list.insert(
                    0,
                    TodayTask {
                        id: task_id,
                        title: trimmed,
                        start: clean_start,
                        deadline: clean_deadline,
                        completed: false,
                        from_template: true,
                    },
                );
            });
        }
    }

    pub fn remove_template(&self, id: &str) {
        self.templates.update(|list| list.retain(|t| t.id != id));
    }
}
