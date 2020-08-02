use super::{DbConnection, Pool};
use crate::models::*;
use actix_web::{web, Responder};
use std::io;

pub async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(Status {
        status: "Up".to_string(),
    })
}

pub async fn get_users() -> impl Responder {
    web::HttpResponse::Ok().json(Status {
        status: "Up".to_string(),
    })
}

pub async fn get_user_by_id() -> impl Responder {
    web::HttpResponse::Ok().json(Status {
        status: "Up".to_string(),
    })
}

pub async fn add_user() -> impl Responder {
    web::HttpResponse::Ok().json(Status {
        status: "Up".to_string(),
    })
}

pub async fn delete_user() -> impl Responder {
    web::HttpResponse::Ok().json(Status {
        status: "Up".to_string(),
    })
}

pub async fn register_user(db: web::Data<Pool>, user_info: web::Json<i32>) -> impl Responder {
    // Ok(
    //     web::block(move || register_user_bg(db, user_id.into_inner()))
    //         .await
    //         .map(|user| HttpResponse::Ok().json(user))
    //         .map_err(|_| HttpResponse::InternalServerError())?,
    // )
    web::HttpResponse::Ok().json(Status {
        status: "Up".to_string(),
    })
}

pub async fn login_user(db: web::Data<Pool>) -> impl Responder {
    web::HttpResponse::Ok().json(Status {
        status: "Up".to_string(),
    })
}

pub async fn logout_user(db: web::Data<Pool>) -> impl Responder {
    web::HttpResponse::Ok().json(Status {
        status: "Up".to_string(),
    })
}

// Registers the user into the database, takes no course of action if the user already exists.
fn register_user_bg(
    pool: web::Data<Pool>,
    api_key: &str,
    favorite_streamer: Option<&str>,
) -> Result<bool, io::Error> {
    let conn = pool.get().unwrap();
    let commander = DBCommands { conn: conn };
    let is_user_exist = commander.is_user_exist(api_key).unwrap_or(false);
    if !is_user_exist {
        let current_user = UserNoId {
            api_key: String::from(api_key),
            favorite_streamer: String::from(favorite_streamer.unwrap_or("NA_None_Null")),
        };
        // Will throw an error if the function errored out for any given reason.
        commander.create_user(current_user).unwrap();
        return Ok(true);
    }
    Ok(false)
}

// pub fn get_all_events(pool: web::Data<Pool>) -> Result<Vec<Event>, diesel::result::Error> {
//     let conn = pool.get().unwrap();
//     let items = events.load::<Event>(&conn)?;
//     Ok(items)
// }
