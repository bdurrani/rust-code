use actix_web::{
    error, guard, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer,
    Result,
};
// this function could be located in different module
// https://github.com/fairingrey/actix-realworld-example-app/blob/master/src/app/mod.rs
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/test")
        .route(web::get().to(|| HttpResponse::Ok()))
        .route(web::head().to(|| HttpResponse::MethodNotAllowed()))
    );
}

/// This could be in another module.
pub fn another(cfg: &mut web::ServiceConfig){
    cfg.service(web::scope("/parrotify")
        .route("/", web::get().to(show_users))
        .route("/{id}", web::get().to(user_detail))
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