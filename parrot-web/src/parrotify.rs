use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;
use validator_derive::Validate;

use actix_web::{
    client::Client, error, guard, middleware, web, App, Error, HttpRequest, HttpResponse,
    HttpServer, Result,
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/parrotify")
            .route("/{text}", web::get().to(show_users))
            .route("/{text}/{repl1}", web::get().to(user_detail)),
    );
}

async fn show_users() -> HttpResponse {
    HttpResponse::Ok().body("Show users")
}

async fn user_detail(path: web::Path<(u32,)>) -> HttpResponse {
    HttpResponse::Ok().body(format!("User detail: {}", path.0))
}

