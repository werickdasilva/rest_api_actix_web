use std::sync::Arc;

use actix_web::{post, web, Responder};
use tokio_postgres::Client;

use super::create_user_dto::UserCreateDto;
use crate::user::user_controller::UserController;

#[post("/user/")]
async fn create_user(
    user_create_dto: web::Json<UserCreateDto>,
    client: web::Data<Arc<Client>>,
) -> impl Responder {
    let user_controller = UserController(client);
    user_controller.create(user_create_dto.0).await
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user);
}
