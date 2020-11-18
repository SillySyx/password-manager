use serde::{Serialize, Deserialize};

use event_sourcing::Event;

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct AddPasswordEvent {
    pub name: String,
    pub password: String,
}

impl AddPasswordEvent {
    pub fn as_event(&self) -> Event {
        let json = serde_json::to_string(self).unwrap();

        Event {
            event_type: String::from("AddPassword"),
            data: json.as_bytes().to_owned(),
        }
    }
}