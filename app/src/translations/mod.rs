pub enum Languages {
    English,
}

pub fn translate(_language: Languages, key: &str) -> String {
    // find translation key in hashmap
    // if found, return translation value

    // else return key
    String::from(key)
}