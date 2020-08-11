mod token_encoder;
mod token_repository;
mod token_service;
pub use token_encoder::*;
pub use token_repository::*;
pub use token_service::*;

use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use uuid::Uuid;

// TokenId
#[derive(Default, Debug, Clone, Eq)]
pub struct TokenId {
    id: String,
}

impl TokenId {
    pub fn new() -> Self {
        let uuid = Uuid::new_v4().to_string();
        TokenId { id: uuid }
    }

    pub fn id(&self) -> &String {
        &self.id
    }
}

impl PartialEq for TokenId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for TokenId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl From<&str> for TokenId {
    fn from(s: &str) -> TokenId {
        TokenId { id: s.to_owned() }
    }
}

// Token
#[derive(Debug, Clone)]
pub struct Token {
    token: String,
}

impl Token {
    pub fn new(token: &str) -> Token {
        Token {
            token: token.to_owned(),
        }
    }

    pub fn token(&self) -> &str {
        &self.token
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.token == other.token
    }
}

// Data
#[derive(Default, Debug, Clone)]
pub struct Data {
    data: HashMap<String, String>,
}

impl Data {
    pub fn new() -> Self {
        Data {
            data: HashMap::new(),
        }
    }

    pub fn add(&mut self, k: &str, v: &str) {
        self.data.insert(k.to_owned(), v.to_owned());
    }

    pub fn get(&self, k: &str) -> Option<&String> {
        self.data.get(&k.to_owned())
    }
}
