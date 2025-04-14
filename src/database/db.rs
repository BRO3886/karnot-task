use crate::models::dao;

pub trait Storage: Clone + Send + Sync {
    fn create_url(&self, url: dao::Url) -> Result<String, String>;
    fn get_url(&self, code: String) -> Result<dao::Url, String>;
}
