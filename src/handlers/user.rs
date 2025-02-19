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

// ... other handlers (get_user_by_id, create_user, update_user, delete_user)
// Similar structure, call services for database logic
