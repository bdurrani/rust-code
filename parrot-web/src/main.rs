use failure::Fail;
use log::debug;
use actix_web::{
    error, guard, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer,
    Result,
};

#[derive(Fail, Debug)]
#[fail(display = "my error")]
pub struct MyError {
    name: &'static str,
}

// Use default implementation for `error_response()` method
impl error::ResponseError for MyError {}

async fn index() -> Result<&'static str, MyError> {
    let err = MyError { name: "test error" };
    debug!("{}", err);
    Err(err)
}

// this function could be located in different module
// https://github.com/fairingrey/actix-realworld-example-app/blob/master/src/app/mod.rs
fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/test")
        .route(web::get().to(|| HttpResponse::Ok()))
        .route(web::head().to(|| HttpResponse::MethodNotAllowed()))
    );
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{middleware::Logger, web, App, HttpServer};

    std::env::set_var("RUST_LOG", "my_errors=debug,actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            .configure(config)
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}