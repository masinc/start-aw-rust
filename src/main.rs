use std::io;

use actix_files::Files;
use actix_web::{App, HttpServer};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    HttpServer::new(||
        App::new().service(Files::new("/", "./").show_files_listing()))
        .bind("localhost:3000")
        .expect("Can not bind to port 3000")
        .run()
        .await
}
