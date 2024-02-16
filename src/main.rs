use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json::Result;

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

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(hello2)
            .service(echo)
            .service(json_resp)
            .route("/hey", web::get().to(manual_hello))
    })
    .workers(4)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
