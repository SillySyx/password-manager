use serde::{Serialize, Deserialize};

use event_sourcing::Event;

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct RemovePasswordEvent {
    pub name: String,
}

impl RemovePasswordEvent {
    pub fn as_event(&self) -> Event {
        let json = serde_json::to_string(self).unwrap();

        Event {
            event_type: String::from("RemovePassword"),
            data: json.as_bytes().to_owned(),
        }
    }
}