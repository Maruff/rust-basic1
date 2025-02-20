use diesel::prelude::*;
use my_user_api::{models::{User, NewUser, UpdateUser}, errors::UserError};

pub fn get_all(conn: &mut DbConnection) -> Result<Vec<User>, diesel::result::Error> {
    users::table.load::<User>(conn)
}

// ... other service functions (get_by_id, create, update, delete)
// These functions contain the database logic.
// They are called by the handlers.

pub fn create(conn: &mut DbConnection, new_user: &NewUser) -> Result<User, diesel::result::Error> {
    diesel::insert_into(users::table)
        .values(new_user) // Insert the NewUser struct (which now has the hash)
        .get_result::<User>(conn)
}

// ... other service functions (get_by_id, update, delete)
