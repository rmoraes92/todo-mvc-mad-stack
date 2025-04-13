use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use r2d2::PooledConnection;
use std::fs::remove_file;
use std::path::Path; // Change the connection type
                     // use diesel::r2d2::{ConnectionManager, Pool};
use diesel_migrations::{
    embed_migrations, EmbeddedMigrations, MigrationHarness,
};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn apply_migrations(conn: &mut SqliteConnection) {
    conn.run_pending_migrations(MIGRATIONS).unwrap();
}

/// encapsulates an extremely primitive workflow to test DAO functions.
/// steps:
/// 1 - remove the test.sqlite3 file
/// 2 - create a new test.sqlite3 file
/// 3 - run migrations
/// 4 - ???
/// 5 - profit
pub fn setup_test_db() -> PooledConnection<ConnectionManager<SqliteConnection>>
{
    // TODO do we need to pool the connections?
    // TODO make this come from dotenvy
    let db_url = ":memory:";
    //cleanup_test_db();
    let manager = ConnectionManager::<SqliteConnection>::new(db_url);
    let mut connection = Pool::builder()
        .build(manager)
        .expect("Failed to create connection pool")
        .get()
        .unwrap();

    apply_migrations(&mut connection);
    return connection;
}

pub fn cleanup_test_db() {
    // TODO rename to reset_test_db?
    // TODO delete entire db?
    let file_path = Path::new("test.sqlite3");
    if file_path.exists() {
        remove_file(file_path).expect("test.sqlite3 does not exists");
    }
}
