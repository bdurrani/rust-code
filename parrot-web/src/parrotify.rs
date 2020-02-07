use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;
use validator_derive::Validate;

use actix_web::{
    client::Client, error, guard, middleware, web, App, Error, HttpRequest, HttpResponse,
    HttpServer, Result,
};
// this function could be located in different module
// https://github.com/fairingrey/actix-realworld-example-app/blob/master/src/app/mod.rs
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| HttpResponse::Ok()))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}

/// This could be in another module.
pub fn another(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/parrotify")
            .route("/", web::get().to(show_users))
            .route("/{id}", web::get().to(user_detail)),
    );
}

async fn index(req: HttpRequest) -> HttpResponse {
    unimplemented!()
}

async fn show_users() -> HttpResponse {
    HttpResponse::Ok().body("Show users")
}

async fn user_detail(path: web::Path<(u32,)>) -> HttpResponse {
    HttpResponse::Ok().body(format!("User detail: {}", path.0))
}

#[derive(Debug, Validate, Deserialize, Serialize)]
struct SomeData {
    #[validate(length(min = 1, max = 1000000))]
    id: String,
    #[validate(length(min = 1, max = 100))]
    name: String,
}

#[derive(Debug, Deserialize)]
struct HttpBinResponse {
    args: HashMap<String, String>,
    data: String,
    files: HashMap<String, String>,
    form: HashMap<String, String>,
    headers: HashMap<String, String>,
    json: SomeData,
    origin: String,
    url: String,
}

/// validate data, post json to httpbin, get it back in the response body, return deserialized
async fn step_x(data: SomeData, client: &Client) -> Result<SomeData, Error> {
    // validate data
    data.validate().map_err(error::ErrorBadRequest)?;

    let mut res = client
        .post("http://httpbin.org/post")
        .send_json(&data)
        .await
        .map_err(Error::from)?; // <- convert SendRequestError to an Error

    let mut body = web::BytesMut::new();
    while let Some(chunk) = res.next().await {
        body.extend_from_slice(&chunk?);
    }
    let body: HttpBinResponse = serde_json::from_slice(&body).unwrap();
    Ok(body.json)
}

async fn create_something(
    some_data: web::Json<SomeData>,
    client: web::Data<Client>,
) -> Result<HttpResponse, Error> {
    println!("{:?}", some_data);
    let some_data_2 = step_x(some_data.into_inner(), &client).await?;
    println!("{:?}", some_data_2);
    let some_data_3 = step_x(some_data_2, &client).await?;
    println!("{:?}", some_data_3);
    let d = step_x(some_data_3, &client).await?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&d).unwrap()))
}

/// This could be in another module.
pub fn async_another(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/something").route("/", web::post().to(create_something)));
}
