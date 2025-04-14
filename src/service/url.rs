use crate::{database::db::Storage, models::dao};

pub struct UrlService<S: Storage> {
    storage: S,
}

impl<S: Storage> UrlService<S> {
    pub fn new(storage: S) -> Self {
        Self { storage }
    }

    pub fn create_url(&self, url: dao::Url) -> Result<String, String> {
        self.storage.create_url(url)
    }

    pub fn get_url(&self, code: String) -> Result<dao::Url, String> {
        self.storage.get_url(code)
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
