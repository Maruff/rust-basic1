use actix_web::{delete, get, patch, post, put, web, Error, HttpResponse};
use diesel::prelude::*;
use my_user_api::{db::DbPool, models::{User, NewUser, UpdateUser}, services::user as user_service};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("User not found")]
    NotFound,
    #[error("Internal Server Error")]
    InternalServerError,
}

impl actix_web::error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UserError::NotFound => HttpResponse::NotFound().finish(),
            UserError::InternalServerError => HttpResponse::InternalServerError().finish(),
        }
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::get().to(get_users))
            .route("/{id}", web::get().to(get_user_by_id))
            .route("", web::post().to(create_user))
            .route("/{id}", web::patch().to(update_user)) // Or PUT for full updates
            .route("/{id}", web::delete().to(delete_user)),
    );
}

#[get("")]
async fn get_users(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let users = web::block(move || user_service::get_all(&mut *conn))
        .await?
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(users))
}

// ... imports

#[post("")]
async fn create_user(
    pool: web::Data<DbPool>,
    new_user: web::Json<NewUser>,
) -> Result<HttpResponse, Error> {

    // 1. Hash the password BEFORE inserting into the database
    let password_hash = // ... Your password hashing logic here (bcrypt, argon2, etc.)
        // Example with bcrypt (add bcrypt crate to Cargo.toml):
        bcrypt::hash(&new_user.password_hash, bcrypt::DEFAULT_COST)
            .map_err(|_| UserError::InternalServerError)?;

    let new_user_with_hash = NewUser {
        name: new_user.name.clone(),
        email: new_user.email.clone(),
        password_hash: password_hash, // Use the hash
    };

    let conn = pool.get().expect("couldn't get db connection from pool");

    let user = web::block(move || user_service::create(&mut *conn, &new_user_with_hash))
        .await?
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Created().json(user))
}

// ... other handlers (get_user_by_id, create_user, update_user, delete_user)
// Similar structure, call services for database logic
