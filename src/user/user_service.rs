use std::sync::Arc;

use actix_web::{
    web::{self, Data},
    HttpResponse, Responder,
};
use tokio_postgres::Client;

use super::{
    create_user_dto::UserCreateDto,
    find_user_dto::FindUserDto,
    user_error::UserError,
    user_query::{CREATE_USER, FIND_USER_BY_ID},
};

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

pub async fn find_by_id(
    client: Data<Arc<Client>>,
    id: i32,
) -> actix_web::Result<web::Json<FindUserDto>, UserError> {
    match client.query(FIND_USER_BY_ID, &[&id]).await {
        Ok(result) => {
            let data: Vec<FindUserDto> = result
                .into_iter()
                .map(|value| FindUserDto::from(value))
                .collect();

            if let Some(find_user) = data.get(0) {
                let user = find_user.clone();
                return Ok(web::Json(user));
            }
            return Err(UserError::NotFoundById);
        }

        Err(_) => return Err(UserError::InternalError),
    }
}
