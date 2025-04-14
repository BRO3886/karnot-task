use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::urls)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Url {
    pub shortcode: String,
    pub link: String,
    pub created_at: std::time::SystemTime,
    pub updated_at: std::time::SystemTime,
}
