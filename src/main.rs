mod user;

use crate::user::create_user_dto::UserCreateDto;
use actix_web::{get, post, web::Json, App, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    String::from("Hello Start Project ")
}

#[post("/user/")]
pub async fn route_create_user(data: Json<UserCreateDto>) -> impl Responder {
    println!("{:#?}", data);
    "Ok"
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(route_create_user))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
