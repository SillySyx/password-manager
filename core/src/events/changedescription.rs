use serde::{Serialize, Deserialize};

use event_sourcing::Event;

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct ChangeDescriptionEvent {
    pub name: String,
    pub new_description: String,
}

impl ChangeDescriptionEvent {
    pub fn as_event(&self) -> Event {
        let json = serde_json::to_string(self).unwrap();

        Event {
            event_type: String::from("ChangeDescription"),
            data: json.as_bytes().to_owned(),
        }
    }
}