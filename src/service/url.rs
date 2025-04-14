use crate::database::db;
use crate::models::dao;
pub struct UrlService {
    storage: Box<dyn db::Storage>,
}

impl UrlService {
    pub fn new(storage: Box<dyn db::Storage>) -> Self {
        Self { storage }
    }

    pub fn create_url(&self, url: dao::Url) -> Result<String, String> {
        self.storage.create_url(url)
    }

    pub fn get_url(&self, code: String) -> Result<dao::Url, String> {
        self.storage.get_url(code)
    }
}

impl Clone for UrlService {
    fn clone(&self) -> Self {
        Self {
            storage: self.storage.clone(),
        }
    }
}
