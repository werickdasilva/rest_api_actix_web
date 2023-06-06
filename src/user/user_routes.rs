use std::sync::Arc;

use actix_web::{get, post, web, Responder};
use tokio_postgres::Client;

use super::create_user_dto::UserCreateDto;
use crate::user::{
    find_user_dto::FindUserDto, user_controller::UserController, user_error::UserError,
};

#[post("/user/")]
async fn create_user(
    user_create_dto: web::Json<UserCreateDto>,
    client: web::Data<Arc<Client>>,
) -> impl Responder {
    let user_controller = UserController(client);
    user_controller.create(user_create_dto.0).await
}

#[get("/user/{id}")]
async fn find_by_id(
    id: web::Path<i32>,
    client: web::Data<Arc<Client>>,
) -> actix_web::Result<web::Json<FindUserDto>, UserError> {
    UserController(client).find_by_id(id.into_inner()).await
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user).service(find_by_id);
}
