use parrotify;

use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use serde_json::json;

// pub fn configure(cfg: &mut web::ServiceConfig) {
//     cfg.service(
//         web::scope("/parrotify")
//             //            .route("/{text}", web::get().to(convert_text))
//             .route("/{text}", web::get().to(convert_text_auto))
//             .route("/{text}/{repl1}", web::get().to(convert_text_param1)),
//     );
// }

#[get("/{content}")]
async fn convert_with_defaults(content: web::Path<String>)-> impl Responder{

    // let content = req.match_info().get("content").unwrap();
    let mut line = parrotify::line::Line::new();

    let v1 = content.to_string();
    for item in v1.chars() {
        line.add_letter(&item);
    }
    line.replace_a(&"-".chars().nth(0).unwrap());
    HttpResponse::Ok().body(format!("{}", line))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    // Use a scope here to avoid duplicating route information.
    cfg.service(
        web::scope("/parrotify")
            .service(convert_with_defaults)
            .service(convert_with_param1)
    );
}

// async fn convert_text(req: HttpRequest) -> HttpResponse {
//     let v1 = req.match_info().get("text").unwrap();
//     let v2 = req.match_info().query("repl1");
//     println!("v1 {:?}", v1);
//     if v2.is_empty() {
//         println!("v2 is empty");
//     } else {
//         println!("v2 {:?}", v2);
//     }
//     HttpResponse::Ok().body("Show users")
// }
//
// async fn convert_text_auto(info: web::Path<String>) -> HttpResponse {
//     let mut line = parrotify::line::Line::new();
//
//     let v1 = info.to_string();
//     for item in v1.chars() {
//         line.add_letter(&item);
//     }
//     line.replace_a(&"-".chars().nth(0).unwrap());
//     println!("v1 {:?}", v1);
//     HttpResponse::Ok().body(format!("{}", line))
// }
//

#[get("/{content}/{param1}")]
async fn convert_with_param1(info: web::Path<(String, String)>) -> HttpResponse {
    let v1 = info.0.to_string(); //req.match_info().get("text").unwrap().to_string();
    let v2 = info.1.to_string(); //req.match_info().query("repl1");
    let mut line = parrotify::line::Line::new();
    info!("v1 {:?} v2 {:?}", v1, v2);
    HttpResponse::Ok().body(format!("{}:{}", v1, v2))
}
