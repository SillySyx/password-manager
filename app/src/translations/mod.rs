use serde_json::Value;

pub enum Languages {
    English,
}

pub fn translate(language: Languages, key: &str) -> String {
    let translations: Value = match language {
        Languages::English => serde_json::from_str(include_str!("translations.en.json")).expect("failed to load translations"),
    };

    match translations[key].as_str() {
        Some(value) => String::from(value),
        None => String::from(key),
    }
}