#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use my_user_api::models::NewUser; // Import your NewUser struct
// ... other imports (bcrypt, your database connection setup, etc.)

fn insert_sample_user(conn: &mut PgConnection) -> Result<(), diesel::result::Error> {
    let password = "my_secret_password";  // In real app, get from user input
    let password_hash = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap(); // Hash the password

    let new_user = NewUser {
        name: "Test User".to_string(),
        email: "test@example.com".to_string(),
        password_hash: password_hash, // Store the hash
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)?;

    Ok(())
}

fn main() -> Result<(), diesel::result::Error> {
    let database_url = "your_database_url"; // Or get from environment variables
    let mut connection = PgConnection::establish(database_url)?;

    insert_sample_user(&mut connection)?;

    Ok(())
}
