use crate::state::AppStore;
use leptos::prelude::*;

impl AppStore {
    pub fn logout(&self) {
        self.current_user_id.set(None);
    }

    pub fn has_any_user(&self) -> bool {
        self.first_register_claimed.get() || !self.users.get().is_empty()
    }
}
