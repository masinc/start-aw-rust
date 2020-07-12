use std::io;

use actix_web::{web, App, HttpRequest, HttpServer, Responder, Result};

async fn hello(req: HttpRequest) -> Result<impl Responder> {
    let to = req.match_info().get("name").unwrap_or("World");
    Ok(format!("Hello {}!", to))
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(hello))
            .service(web::resource("/{name}").to(hello))
    })
    .bind("localhost:3000")
    .expect("Can not bind to port 3000")
    .run()
    .await
}
