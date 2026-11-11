use crate::domain::{UserAccount, UserRole};
use crate::state::AppStore;
use leptos::prelude::*;

impl AppStore {
    pub fn register_first_admin(
        &self,
        username: String,
        password: String,
    ) -> Result<(), String> {
        if self.first_register_claimed.get_untracked() {
            return Err("Initial admin already registered. Sign in at /login.".to_string());
        }
        let clean_user = username.trim().to_string();
        if clean_user.len() < 3 {
            return Err("Username must be at least 3 characters.".to_string());
        }
        if clean_user.len() > 32 {
            return Err("Username cannot exceed 32 characters.".to_string());
        }
        if password.trim().len() < 4 {
            return Err("Password must be at least 4 characters.".to_string());
        }
        if password.len() > 128 {
            return Err("Password cannot exceed 128 characters.".to_string());
        }
        let id = self.next_id.get_untracked();
        self.next_id.set(id + 1);
        let account = UserAccount {
            id,
            username: clean_user,
            password,
            role: UserRole::Admin,
            is_first_admin: true,
        };
        self.users.update(|list| {
            for existing in list.iter_mut() {
                existing.is_first_admin = false;
            }
            list.insert(0, account);
        });
        self.current_user_id.set(Some(id));
        self.first_register_claimed.set(true);
        Ok(())
    }

    pub fn login(&self, username: String, password: String) -> Result<(), String> {
        let clean = username.trim().to_lowercase();
        if clean.is_empty() || password.is_empty() {
            return Err("Enter both username and password.".to_string());
        }
        let matched = self
            .users
            .get_untracked()
            .into_iter()
            .find(|u| u.username.to_lowercase() == clean && u.password == password);

        match matched {
            Some(account) => {
                self.current_user_id.set(Some(account.id));
                Ok(())
            }
            None => Err("Invalid username or password.".to_string()),
        }
    }

    pub fn logout(&self) {
        self.current_user_id.set(None);
    }

    pub fn has_any_user(&self) -> bool {
        !self.users.get().is_empty()
    }

    pub fn reset_to_first_boot(&self) {
        self.users.set(Vec::new());
        self.current_user_id.set(None);
        self.first_register_claimed.set(false);
    }

    pub fn admin_create_user(
        &self,
        username: String,
        password: String,
        as_admin: bool,
    ) -> Result<(), String> {
        if !self.is_admin() {
            return Err("Admin privileges required.".to_string());
        }
        let clean = username.trim().to_string();
        if clean.len() < 3 {
            return Err("Username must be at least 3 characters.".to_string());
        }
        if clean.len() > 32 {
            return Err("Username cannot exceed 32 characters.".to_string());
        }
        if password.trim().len() < 4 {
            return Err("Password must be at least 4 characters.".to_string());
        }
        if password.len() > 128 {
            return Err("Password cannot exceed 128 characters.".to_string());
        }
        let exists = self
            .users
            .get_untracked()
            .iter()
            .any(|u| u.username.eq_ignore_ascii_case(&clean));
        if exists {
            return Err("A user with that username already exists.".to_string());
        }
        let id = self.next_id.get_untracked();
        self.next_id.set(id + 1);
        self.users.update(|list| {
            list.push(UserAccount {
                id,
                username: clean,
                password,
                role: if as_admin {
                    UserRole::Admin
                } else {
                    UserRole::User
                },
                is_first_admin: false,
            });
        });
        Ok(())
    }

    pub fn admin_toggle_role(&self, target_id: u64) {
        if !self.is_admin() {
            return;
        }
        self.users.update(|list| {
            if let Some(u) = list.iter_mut().find(|u| u.id == target_id) {
                if !u.is_first_admin {
                    u.role = match u.role {
                        UserRole::Admin => UserRole::User,
                        UserRole::User => UserRole::Admin,
                    };
                }
            }
        });
    }

    pub fn admin_remove_user(&self, target_id: u64) {
        if !self.is_admin() {
            return;
        }
        self.users
            .update(|list| list.retain(|u| u.id != target_id || u.is_first_admin));
    }
}
