use actix_web::{
    get,
    HttpResponse, 
    Responder,
};


#[get("/api/user")]
pub async fn get_user() -> impl Responder {
    HttpResponse::Ok().body("user")
}

