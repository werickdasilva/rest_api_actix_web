use actix_web::{web, Responder};
use std::sync::Arc;
use tokio_postgres::Client;

use super::{create_user_dto::UserCreateDto, user_service};

pub struct UserController(pub web::Data<Arc<Client>>);

impl UserController {
    pub async fn create(self, user_create_dto: UserCreateDto) -> impl Responder {
        user_service::create(self.0, user_create_dto).await
    }
}
