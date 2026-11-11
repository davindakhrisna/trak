pub mod auth_ops;
pub mod seed;

use crate::domain::{RecurringTemplate, ThemeMode, TodayTask, UserAccount, Weekday};
use leptos::prelude::*;

#[derive(Clone, Copy)]
pub struct AppStore {
    pub users: RwSignal<Vec<UserAccount>>,
    pub current_user_id: RwSignal<Option<u64>>,
    pub first_register_claimed: RwSignal<bool>,
    pub today_tasks: RwSignal<Vec<TodayTask>>,
    pub templates: RwSignal<Vec<RecurringTemplate>>,
    pub selected_day: RwSignal<Weekday>,
    pub next_id: RwSignal<u64>,
    pub theme_mode: RwSignal<ThemeMode>,
}

pub fn provide_app_store() -> AppStore {
    let templates_init = seed::default_templates();
    let today_init = seed::default_today_tasks(&templates_init);
    let store = AppStore {
        users: RwSignal::new(seed::default_users()),
        current_user_id: RwSignal::new(Some(1)),
        first_register_claimed: RwSignal::new(false),
        today_tasks: RwSignal::new(today_init),
        templates: RwSignal::new(templates_init),
        selected_day: RwSignal::new(Weekday::today()),
        next_id: RwSignal::new(500),
        theme_mode: RwSignal::new(ThemeMode::System),
    };
    provide_context(store);
    store
}

pub fn use_app_store() -> AppStore {
    expect_context::<AppStore>()
}

impl AppStore {
    fn alloc_id(&self) -> u64 {
        let id = self.next_id.get_untracked();
        self.next_id.set(id + 1);
        id
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

    pub fn add_today_task(&self, title: String) {
        let mut trimmed = title.trim().to_string();
        if trimmed.is_empty() {
            return;
        }
        if trimmed.len() > 200 {
            trimmed.truncate(200);
        }
        let id = self.alloc_id();
        self.today_tasks.update(|list| {
            list.insert(
                0,
                TodayTask {
                    id,
                    title: trimmed,
                    completed: false,
                    from_template: false,
                },
            );
        });
    }

    pub fn toggle_today_task(&self, id: u64) {
        self.today_tasks.update(|list| {
            if let Some(item) = list.iter_mut().find(|t| t.id == id) {
                item.completed = !item.completed;
            }
        });
    }

    pub fn remove_today_task(&self, id: u64) {
        self.today_tasks.update(|list| list.retain(|t| t.id != id));
    }

    pub fn add_template(&self, day: Weekday, title: String) {
        let mut trimmed = title.trim().to_string();
        if trimmed.is_empty() {
            return;
        }
        if trimmed.len() > 200 {
            trimmed.truncate(200);
        }
        let id = self.alloc_id();
        self.templates.update(|list| {
            list.push(RecurringTemplate {
                id,
                day,
                title: trimmed.clone(),
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
                        completed: false,
                        from_template: true,
                    },
                );
            });
        }
    }

    pub fn remove_template(&self, id: u64) {
        self.templates.update(|list| list.retain(|t| t.id != id));
    }
}
