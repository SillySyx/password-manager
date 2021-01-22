use serde::{Serialize, Deserialize};

use event_sourcing::Event;

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct ChangeCategoryEvent {
    pub name: String,
    pub new_category: String,
}

impl ChangeCategoryEvent {
    pub fn as_event(&self) -> Event {
        let json = serde_json::to_string(self).unwrap();

        Event {
            event_type: String::from("ChangeCategory"),
            data: json.as_bytes().to_owned(),
        }
    }
}