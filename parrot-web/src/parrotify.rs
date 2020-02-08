//use futures::StreamExt;
//use serde::{Deserialize, Serialize};
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
            .route("/{text}", web::get().to(convert_text))
            .route("/{text}/{repl1}", web::get().to(convert_text_param1)),
    );
}

async fn convert_text(req: HttpRequest) -> HttpResponse {
    let v1 = req.match_info().get("text").unwrap();
    let v2 = req.match_info().query("repl1");
    println!("v1 {:?}", v1);
    if v2.is_empty() {
        println!("v2 is empty");
    } else {
        println!("v2 {:?}", v2);
    }
    HttpResponse::Ok().body("Show users")
}

async fn convert_text_param1(req: HttpRequest) -> HttpResponse {
    let v1 = req.match_info().get("text").unwrap();
    let v2 = req.match_info().query("repl1");
    println!("v1 {:?} v2 {:?}", v1, v2);
    HttpResponse::Ok().body("Show users with param1")
}

//async fn user_detail(path: web::Path<(u32,)>) -> HttpResponse {
//    HttpResponse::Ok().body(format!("User detail: {}", path.0))
//}

