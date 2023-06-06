use actix_web::{
    get, post,
    web::{self, Data, Json, Path, ServiceConfig},
    Responder,
};
use std::sync::Arc;
use tokio_postgres::Client;

use crate::user::{
    dto::{create_dto_user::CreateDtoUser, find_user_dto::FindUserDto},
    user_error::UserError,
    user_service::UserService,
};

#[post("/user/")]
async fn create(client: Data<Arc<Client>>, user_create_dto: Json<CreateDtoUser>) -> impl Responder {
    UserService::new(client).create(user_create_dto.0).await
}

#[get("/user/{id}")]
async fn find_by_id(
    client: Data<Arc<Client>>,
    id: Path<i32>,
) -> actix_web::Result<web::Json<FindUserDto>, UserError> {
    UserService::new(client).find_by_id(id.abs()).await
}

pub fn get_routes(cfg: &mut ServiceConfig) {
    cfg.service(create).service(find_by_id);
}
