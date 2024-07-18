use crate::error_handler::LandeedError;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use lazy_static::lazy_static;
use r2d2;
use std::env;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

lazy_static! {
    static ref POOL: Pool = {
        let db_url = env::var("DATABASE_URL").expect("Database url not set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("Failed to create db pool")
    };
}

pub fn connection() -> Result<DbConnection, LandeedError> {
    Ok(POOL.get()?)
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn run_migration(conn: &mut PgConnection) {
    conn.run_pending_migrations(MIGRATIONS).unwrap();
}
