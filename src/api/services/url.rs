pub struct UrlService {
    storage: db::Storage,
    connection: PgConnection,
}

impl UrlService {
    pub fn create_url(&self, url: dao::Url) -> String {
        "".to_string()
    }
}
