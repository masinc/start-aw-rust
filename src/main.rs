use actix_web::{error, web, App, HttpResponse, HttpServer, Result};
use serde_derive::*;
use std::io;
use tera::{Context, Tera};

struct AppState {
    template: Tera,
}

#[derive(Deserialize)]
struct HelloPath {
    name: String,
}

async fn hello_template(
    app: web::Data<AppState>,
    path: web::Path<HelloPath>,
) -> Result<HttpResponse> {
    let mut context = Context::new();
    context.insert("name", &path.name);

    let body = app
        .template
        .render("index.html.tera", &context)
        .map_err(|e| error::ErrorInternalServerError(format!("{}", e)))?;
    Ok(HttpResponse::Ok().body(body))
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        let app = AppState {
            template: Tera::new("templates/**/*").unwrap(),
        };
        App::new()
            .data(app)
            .route("/{name}", web::get().to(hello_template))
    })
    .bind("localhost:3000")
    .expect("Can not bind to port 3000")
    .run()
    .await
}
