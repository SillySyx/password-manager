use serde::{Serialize, Deserialize};

use event_sourcing::Event;

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct ChangeNameEvent {
    pub name: String,
    pub new_name: String,
}

impl ChangeNameEvent {
    pub fn as_event(&self) -> Event {
        let json = serde_json::to_string(self).unwrap();

        Event {
            event_type: String::from("ChangeName"),
            data: json.as_bytes().to_owned(),
        }
    }
}