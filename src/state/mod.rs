pub mod auth_ops;

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

impl AppStore {
    pub fn current_user(&self) -> Option<UserAccount> {
        let uid = self.current_user_id.get()?;
        self.users.get().into_iter().find(|u| u.id == uid)
    }

    pub fn is_admin(&self) -> bool {
        self.current_user()
            .map(|u| u.role.is_admin())
            .unwrap_or(false)
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

    pub fn remove_template(&self, id: &str) {
        self.templates.update(|list| list.retain(|t| t.id != id));
    }
}
