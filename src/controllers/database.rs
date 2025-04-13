use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::{sqlite::SqliteConnection, ConnectionError};
use r2d2::Error;

pub type DatabaseConnection = SqliteConnection;
pub type DatabaseConnectionError = ConnectionError;
pub type DatabaseConnectionManager = ConnectionManager<SqliteConnection>;
pub type DatabaseConnectionPool = Pool<ConnectionManager<SqliteConnection>>;
pub type DatabaseConnectionPoolError = Error;

pub fn build_db_conn_pool(
    db_url: &str,
) -> Result<DatabaseConnectionPool, DatabaseConnectionPoolError> {
    let conn_man: DatabaseConnectionManager =
        ConnectionManager::<SqliteConnection>::new(db_url);
    Pool::builder().test_on_check_out(true).build(conn_man)
}
