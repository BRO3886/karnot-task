// @generated automatically by Diesel CLI.

diesel::table! {
    urls (shortcode) {
        #[max_length = 255]
        shortcode -> Varchar,
        #[max_length = 255]
        link -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        uses -> Int4,
    }
}
