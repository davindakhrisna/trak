use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Weekday {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}

impl Weekday {
    pub const ALL: [Weekday; 7] = [
        Weekday::Mon,
        Weekday::Tue,
        Weekday::Wed,
        Weekday::Thu,
        Weekday::Fri,
        Weekday::Sat,
        Weekday::Sun,
    ];

    pub fn code(self) -> &'static str {
        match self {
            Weekday::Mon => "MON",
            Weekday::Tue => "TUE",
            Weekday::Wed => "WED",
            Weekday::Thu => "THU",
            Weekday::Fri => "FRI",
            Weekday::Sat => "SAT",
            Weekday::Sun => "SUN",
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Weekday::Mon => "Monday",
            Weekday::Tue => "Tuesday",
            Weekday::Wed => "Wednesday",
            Weekday::Thu => "Thursday",
            Weekday::Fri => "Friday",
            Weekday::Sat => "Saturday",
            Weekday::Sun => "Sunday",
        }
    }

    pub fn today() -> Self {
        use chrono::Datelike;
        match chrono::Local::now().weekday() {
            chrono::Weekday::Mon => Weekday::Mon,
            chrono::Weekday::Tue => Weekday::Tue,
            chrono::Weekday::Wed => Weekday::Wed,
            chrono::Weekday::Thu => Weekday::Thu,
            chrono::Weekday::Fri => Weekday::Fri,
            chrono::Weekday::Sat => Weekday::Sat,
            chrono::Weekday::Sun => Weekday::Sun,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserRole {
    Admin,
    User,
}

impl UserRole {
    pub fn is_admin(self) -> bool {
        matches!(self, UserRole::Admin)
    }

    pub fn label(self) -> &'static str {
        match self {
            UserRole::Admin => "ADMIN",
            UserRole::User => "USER",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserAccount {
    pub id: String,
    pub username: String,
    pub password: String,
    pub role: UserRole,
    pub is_first_admin: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TodayTask {
    pub id: String,
    pub title: String,
    pub start: Option<String>,
    pub deadline: Option<String>,
    pub completed: bool,
    pub from_template: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecurringTemplate {
    pub id: String,
    pub day: Weekday,
    pub title: String,
    pub start: Option<String>,
    pub deadline: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ThemeMode {
    #[default]
    System,
    Light,
    Dark,
}

impl ThemeMode {
    pub fn as_str(self) -> &'static str {
        match self {
            ThemeMode::System => "system",
            ThemeMode::Light => "light",
            ThemeMode::Dark => "dark",
        }
    }

    pub fn from_str_opt(s: &str) -> Option<Self> {
        match s {
            "light" => Some(ThemeMode::Light),
            "dark" => Some(ThemeMode::Dark),
            "system" => Some(ThemeMode::System),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserSettings {
    pub user_id: String,
    pub ntfy_enabled: bool,
    pub ntfy_server: String,
    pub ntfy_topic: String,
    pub notify_start: bool,
    pub notify_1h: bool,
    pub notify_25m: bool,
    pub notify_crossed: bool,
    pub lockdown_enabled: bool,
    pub lockdown_secret: String,
    pub locked_until: Option<String>,
    pub missed_deadlines_today: i64,
    pub timezone: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LockdownStatus {
    pub locked: bool,
    pub locked_until: Option<String>,
    pub missed_deadlines: i64,
    pub reason: String,
}

pub fn clean_err(err: impl ToString) -> String {
    let mut s = err.to_string();
    let prefixes = [
        "error running server function: ",
        "error running server function:",
        "Server function error: ",
        "Server function error:",
    ];
    let mut changed = true;
    while changed {
        changed = false;
        for p in prefixes {
            if s.starts_with(p) {
                s = s[p.len()..].trim().to_string();
                changed = true;
                break;
            }
        }
    }
    s
}
