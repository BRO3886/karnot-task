use url::Url;

pub fn validate_url(link: &str) -> bool {
    match Url::parse(link) {
        Ok(_) => true,
        Err(_) => false,
    }
}
