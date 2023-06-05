use std::sync::Arc;

use actix_web::{web::Data, Responder};
use tokio_postgres::Client;

use super::{create_user_dto::UserCreateDto, user_query::CREATE_USER};

pub async fn create(client: Data<Arc<Client>>, user_create_dto: UserCreateDto) -> impl Responder {
    match client
        .query(
            CREATE_USER,
            &[
                &user_create_dto.name,
                &user_create_dto.email,
                &user_create_dto.password,
            ],
        )
        .await
    {
        Ok(_) => return "Sucess",
        Err(_) => return "Error",
    };
}
