use diesel::mysql::MysqlConnection;
use diesel::r2d2::{self, ConnectionManager};

pub type DbConnection = MysqlConnection;
pub type DbPool = r2d2::Pool<ConnectionManager<DbConnection>>;

pub fn create_pool(database_url: &str) -> Result<DbPool, diesel::r2d2::Error> {
    let manager = ConnectionManager::<DbConnection>::new(database_url);
    diesel::r2d2::Pool::builder().build(manager).map_err(|e| {
        diesel::r2d2::Error::from(e) // Use to_string() to convert
    })
}
