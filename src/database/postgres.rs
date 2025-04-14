use crate::schema::urls;

use crate::{database::db, models::dao};
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use std::env;
use std::error::Error;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
type DB = Pg;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct PostgresStorage {
    pool: DbPool,
}

impl db::Storage for PostgresStorage {
    fn create_url(&self, url: dao::Url) -> Result<String, String> {
        let conn = &mut self.pool.get().unwrap();
        diesel::insert_into(urls::table)
            .values(&url)
            .execute(conn)
            .map_err(|err| {
                println!("Error creating url: {}", err);
                err.to_string()
            })?;
        Ok(url.shortcode)
    }

    fn get_url(&self, code: String) -> Result<dao::Url, String> {
        let conn = &mut self.pool.get().unwrap();
        let url = urls::table
            .filter(urls::shortcode.eq(code))
            .first::<dao::Url>(conn)
            .map_err(|err| {
                println!("Error getting url: {}", err);
                err.to_string()
            })?;
        Ok(url)
    }
}

impl PostgresStorage {
    pub fn new() -> Self {
        let pool = r2d2::Pool::builder()
            .build(ConnectionManager::new(
                env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            ))
            .expect("Failed to create pool.");

        run_migrations(&mut pool.get().unwrap()).unwrap_or_else(|e| {
            println!("Error running migrations: {}", e);
            std::process::exit(1);
        });

        Self { pool }
    }
}

fn run_migrations(
    connection: &mut impl MigrationHarness<DB>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|err| panic!("Error connecting to {}: err: {}", database_url, err))
}
