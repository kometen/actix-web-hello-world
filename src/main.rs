use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result, HttpRequest};
use actix_web::http::{header};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde::{Serialize, Deserialize};

#[cfg(test)]
mod test;

#[derive(Serialize, Deserialize)]
struct Info {
    user_id: u32,
    username: String,
}

async fn _index(req: HttpRequest) -> HttpResponse {
    if let Some(_hdr) = req.headers().get(header::CONTENT_TYPE) {
        HttpResponse::Ok().into()
    } else {
        HttpResponse::BadRequest().into()
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hej med dig!")
}

#[get("/users/{user_id}/{username}")]
async fn user(info: web::Path<Info>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(Info {
        user_id: info.user_id, username: info.username.to_string(),
    }))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body.to_lowercase())
}

#[post("/identify")]
async fn identify(info: web::Json<Info>) -> Result<String> {
    Ok(format!("Welcome {}, your id is {}", info.username, info.user_id))
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("Jeg æder blåbærsyltetøj!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(user)
            .service(identify)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind_openssl("127.0.0.1:8080", builder)?
        .run()
        .await
}
