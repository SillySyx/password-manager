use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn generate_passphrase() -> Result<String, Box::<dyn std::error::Error>> {
    let mut rng = thread_rng();

    let mut adjectives = list_adjectives();
    let mut nouns = list_nouns();

    adjectives.shuffle(&mut rng);
    nouns.shuffle(&mut rng);

    Ok(format!("{} {} {} {} {} {}", adjectives[0], nouns[0], adjectives[1], nouns[1], adjectives[2], nouns[2]))
}

fn list_adjectives() -> Vec<String> {
    let source: &str = include_str!("adjectives");

    source
        .split(|c| c == '\r' || c == '\n')
        .map(|s| s.to_owned())
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>()
}

fn list_nouns() -> Vec<String> {
    let source: &str = include_str!("nouns");

    source
        .split(|c| c == '\r' || c == '\n')
        .map(|s| s.to_owned())
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>()
}