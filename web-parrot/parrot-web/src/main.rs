#[macro_use] extern crate log;

use actix_web::{client::Client, error, Result};
use failure::Fail;
use listenfd::ListenFd;
use std::env;
use dotenv::dotenv;

mod parrotify_config;
mod parrotify;
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
    dotenv().ok();
    env_logger::init();

    let mut listenfd = ListenFd::from_env();

    info!("Starting server on {} on port {}", BIND_IP, BIND_PORT);
    let mut server = HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(Logger::default())
            .data(Client::default())
            .configure(parrotify_config::configure)
            .route("/", web::get().to(index))
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(format!("{}:{}", BIND_IP, BIND_PORT))?,
    };

    server.run().await
}
