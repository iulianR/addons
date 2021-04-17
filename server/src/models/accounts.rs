use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::redacted::Redacted;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterPost {
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: Redacted<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: Redacted<String>,
}

impl Account {
    pub fn new(username: String, email: String, password: String) -> Account {
        Account {
            id: Uuid::new_v4(),
            username,
            email,
            password: Redacted::_new(password),
        }
    }
}
