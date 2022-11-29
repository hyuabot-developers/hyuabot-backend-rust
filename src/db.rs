use crate::error_handler::CustomError;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;
use r2d2;
use std::env;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DBConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");
lazy_static!{
    pub static ref DB_POOL: Pool = {
        let host: String = env::var("POSTGRES_HOST").expect("POSTGRES_HOST must be set");
        let port: String = env::var("POSTGRES_PORT").expect("POSTGRES_PORT must be set");
        let user: String = env::var("POSTGRES_ID").expect("POSTGRES_ID must be set");
        let password: String = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
        let database_name: String = env::var("POSTGRES_DB").expect("POSTGRES_DB must be set");
        let db_url = format!("postgres://{}:{}@{}:{}/{}", user, password, host, port, database_name);
        let connection_manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(connection_manager).expect("Failed to create pool.")
    };
}


pub fn initialize_connection_pool() {
    lazy_static::initialize(&DB_POOL);
    let mut conn = establish_connection();
    conn.run_pending_migrations(MIGRATIONS).unwrap();
}

pub fn establish_connection() -> Result<DBConnection, CustomError>{
    DB_POOL.get().map_err(|e| CustomError::new(500, format!("Failed to get database connection: {}", e)))
}