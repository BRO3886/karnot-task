use chrono::{DateTime, Utc};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

use crate::models::dao;

const ALPHABET: [char; 62] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9',
];

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUrlRequest {
    pub link: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUrlResponse {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Url {
    id: String,
    link: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Url {
    pub fn new(link: String) -> Self {
        let id = nanoid!(10, &ALPHABET);
        Self {
            id,
            link,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_link(&self) -> String {
        self.link.clone()
    }

    pub fn from_db(url: dao::Url) -> Self {
        Self {
            id: url.shortcode,
            link: url.link,
            created_at: DateTime::from(url.created_at),
            updated_at: DateTime::from(url.updated_at),
        }
    }
}

impl From<Url> for dao::Url {
    fn from(url: Url) -> Self {
        dao::Url {
            shortcode: url.id,
            link: url.link,
            created_at: url.created_at.into(),
            updated_at: url.updated_at.into(),
        }
    }
}
