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
    Result
};
use std::env;
use api::user::get_user;
use websockets::ws::ws_index;
use actix_files as fs;

#[get("/")]
async fn hello() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("public/index.html")?)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

const WORKER_COUNT: usize = 4;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let exe_path = env::current_exe().unwrap();
    env::set_current_dir(&exe_path.parent().unwrap()).unwrap();
    println!("{:?}", env::current_dir().unwrap());

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)            
            .service(get_user)
            .service(fs::Files::new("/app", "./public"))
            .route("/ws/", web::get().to(ws_index))
    })
    .workers(WORKER_COUNT)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}