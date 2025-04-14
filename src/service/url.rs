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
            .map_err(|err| {
                println!("Error getting url: {}", err);
                "URL not found".to_string()
            })
            .map(dto::Url::from_db)
    }

    pub fn increment_uses(&self, code: String) -> Result<i32, String> {
        self.storage.increment_uses(code).map_err(|err| {
            println!("Error incrementing uses: {}", err);
            "Failed to increment uses".to_string()
        })
    }
}

impl<S: Storage + Clone> Clone for UrlService<S> {
    fn clone(&self) -> Self {
        Self {
            storage: self.storage.clone(),
        }
    }
}
