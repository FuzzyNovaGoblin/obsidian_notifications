use crate::file_scanner::time_reminder::model::ReminderKey;
use shared_singleton::*;
use std::collections::HashMap;

pub struct State {
    pub reminders: HashMap<String, Vec<ReminderKey>>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            reminders: Default::default(),
        }
    }
}

impl_singleton_arc_mutex_tokio!(State, State::default());
