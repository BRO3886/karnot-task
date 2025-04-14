use crate::{database::db::Storage, models::dto};

pub struct UrlService<S: Storage> {
    storage: S,
}

impl<S: Storage> UrlService<S> {
    pub fn new(storage: S) -> Self {
        Self { storage }
    }

    pub fn create_url(&self, url: dto::Url) -> Result<String, String> {
        self.storage.create_url(url.into())
    }

    pub fn get_url(&self, code: String) -> Result<dto::Url, String> {
        self.storage
            .get_url(code)
            .map_err(|_| "URL not found".to_string())
            .map(dto::Url::from_db)
    }
}

// Clone is automatically implemented if S: Clone
impl<S: Storage + Clone> Clone for UrlService<S> {
    fn clone(&self) -> Self {
        Self {
            storage: self.storage.clone(),
        }
    }
}
