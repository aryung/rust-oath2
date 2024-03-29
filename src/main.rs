use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::env;

#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
    age: i32,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/json/{name}")]
async fn json_resp(name: web::Path<String>) -> Result<impl Responder> {
    let obj = MyObj {
        name: name.to_string(),
        age: 30,
    };
    Ok(web::Json(obj))
}

#[get("/hello2")]
async fn hello2(query: web::Query<MyObj>) -> impl Responder {
    let resp = format!("Hello, {}! age: {}", query.name, query.age);
    HttpResponse::Ok().body(resp)
}

#[get("/hello3")]
async fn hello3() -> impl Responder {
    HttpResponse::TemporaryRedirect()
        .header("Location", "http://example.com")
        .finish()
}

#[get("/callback")]
async fn callback() -> impl Responder {
    HttpResponse::TemporaryRedirect()
        .header(
            "Location",
            "https://03d3-2001-b011-8-5f21-d893-708c-2bbf-3b2d.ngrok-free.app",
        )
        .finish()
}

#[derive(Serialize, Deserialize)]
struct AuthCode {
    code: String,
    state: String,
}

#[get("/auth")]
async fn auth(auth_code: web::Query<AuthCode>) -> impl Responder {
    let resp = format!("code: {}, state: {}", auth_code.code, auth_code.state);
    println!("auth: {}", resp);
    HttpResponse::TemporaryRedirect()
        .header(
            "Location",
            "https://03d3-2001-b011-8-5f21-d893-708c-2bbf-3b2d.ngrok-free.app",
        )
        .finish()
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port = env::var("APP_PORT").unwrap_or_else(|_| "8080".to_string());
    let server_address = format!("127.0.0.1:{}", port);

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(hello2)
            .service(hello3)
            .service(callback)
            .service(auth)
            .service(echo)
            .service(json_resp)
            .route("/hey", web::get().to(manual_hello))
    })
    .workers(4)
    .bind(&server_address)?
    .run()
    .await
}
