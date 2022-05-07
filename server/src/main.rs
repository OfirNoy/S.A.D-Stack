mod api;
mod websockets;
mod data;

use actix_web::{
    get, 
    post, 
    web, 
    App, 
    HttpResponse, 
    HttpServer, 
    Responder,
    HttpRequest, 
    Result
};

use api::user::get_user;
use websockets::ws::ws_index;
use actix_files::NamedFile;
use std::path::PathBuf;

async fn index(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[get("/")]
async fn hello() -> Result<NamedFile> {
    Ok(NamedFile::open("index.html")?)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

const WORKER_COUNT: usize = 4;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)            
            .service(get_user)
            .route("/ws/", web::get().to(ws_index))            
            .route("/{filename:.*}", web::get().to(index))
    })
    .workers(WORKER_COUNT)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}