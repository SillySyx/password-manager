use serde::{Serialize, Deserialize};

use event_sourcing::Event;

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct ChangePasswordEvent {
    pub name: String,
    pub new_password: String,
}

impl ChangePasswordEvent {
    pub fn as_event(&self) -> Event {
        let json = serde_json::to_string(self).unwrap();

        Event {
            event_type: String::from("ChangePassword"),
            data: json.as_bytes().to_owned(),
        }
    }
}