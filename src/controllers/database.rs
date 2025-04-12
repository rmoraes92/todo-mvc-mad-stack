use diesel::r2d2::ConnectionManager;
//use diesel::r2d2::Error;
use diesel::r2d2::Pool;
use diesel::{ConnectionError, sqlite::SqliteConnection};
use dotenvy::dotenv;
use r2d2::Error;
use std::env;

pub type DatabaseConnection = SqliteConnection;
pub type DatabaseConnectionError = ConnectionError;
pub type DatabaseConnectionManager = ConnectionManager<SqliteConnection>;
pub type DatabaseConnectionPool = Pool<ConnectionManager<SqliteConnection>>;
pub type DatabaseConnectionPoolError = Error;

//pub async fn build_db_conn() -> Result<DatabaseConnection, DatabaseConnectionError> {
//    dotenv().ok();
//
//    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//
//    r2d2::ConnectionManager::<SqliteConnection>::new(database_url)
//}

pub async fn build_db_conn_pool() -> Result<DatabaseConnectionPool, DatabaseConnectionPoolError> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn_man: DatabaseConnectionManager = ConnectionManager::<SqliteConnection>::new(db_url);
    Pool::builder().test_on_check_out(true).build(conn_man)
}

//pub type DatabaseConnection = SyncConnectionWrapper<SqliteConnection>;
//pub type DatabaseConnectionError = ConnectionError;
//pub type DatabaseConnectionPool = Pool<SyncConnectionWrapper<SqliteConnection>>;
//pub type DatabaseConnectionPoolError = BuildError;
//
//pub async fn build_db_conn_pool() -> Result<DatabaseConnectionPool, DatabaseConnectionPoolError> {
//    dotenv().ok();
//
//    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//    let manager = AsyncDieselConnectionManager::<DatabaseConnection>::new(db_url);
//    DatabaseConnectionPool::builder(manager).build()
//}
