use std::time::SystemTime;

use crate::models::dao;

use super::db::Storage;

#[derive(Clone)]
pub struct RedisStorage;

impl Storage for RedisStorage {
    fn create_url(&self, url: dao::Url) -> Result<String, String> {
        Ok(url.shortcode)
    }

    fn get_url(&self, code: String) -> Result<dao::Url, String> {
        Ok(dao::Url {
            shortcode: code,
            link: "".to_string(),
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
            uses: 0,
        })
    }

    fn increment_uses(&self, _: String) -> Result<i32, String> {
        Ok(0)
    }
}
