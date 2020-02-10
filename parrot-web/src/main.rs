use actix_web::{client::Client, error, Result};
use failure::Fail;
use log::debug;
use parrotify;

mod parrotify_config;
mod samples;

const BIND_IP: &str = "127.0.0.1";
const BIND_PORT: &str = "8088";

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

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{middleware::Logger, web, App, HttpServer};

    std::env::set_var("RUST_LOG", "my_errors=debug,actix_web=debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let _ = parrotify::line::Line::new();
    println!("Starting server on {} on port {}", BIND_IP, BIND_PORT);
    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(Logger::default())
            .data(Client::default())
            .configure(parrotify_config::configure)
            .route("/", web::get().to(index))
    })
    .bind(format!("{}:{}", BIND_IP, BIND_PORT))?
    .run()
    .await
}
