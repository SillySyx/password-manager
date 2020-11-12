use event_sourcing::BinaryState;

#[derive(Debug, Clone)]
pub struct Password {
    pub name: String,
    pub password: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct PasswordsState {
    pub passwords: Vec<Password>,
}

impl BinaryState for PasswordsState {
    fn from_bytes(_bytes: Vec<u8>) -> Self {
        Self {
            passwords: vec![],
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![]
    }
}