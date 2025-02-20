// use diesel::pg::PgConnection;
// use diesel::r2d2::{self, ConnectionManager};

// pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

// pub fn create_pool(database_url: &str) -> Result<DbPool, r2d2::Error> {
//     let manager = ConnectionManager::<PgConnection>::new(database_url);
//     r2d2::Pool::builder().build(manager)
// }


pub type DbPool = r2d2::Pool<ConnectionManager<DbConnection>>;

pub fn create_pool(database_url: &str) -> Result<DbPool, r2d2::Error> {
    let manager = ConnectionManager::<DbConnection>::new(database_url);
    r2d2::Pool::builder().build(manager)
}
