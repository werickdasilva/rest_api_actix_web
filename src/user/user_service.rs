use std::sync::Arc;

use actix_web::{
    web::{self, Data},
    Responder,
};
use tokio_postgres::Client;

use super::{
    dto::{create_dto_user::CreateDtoUser, find_user_dto::FindUserDto},
    user_error::UserError,
    user_query::{CREATE_USER, FIND_USER_BY_ID},
};

pub struct UserService {
    client: Data<Arc<Client>>,
}

impl UserService {
    pub fn new(client: Data<Arc<Client>>) -> Self {
        Self { client }
    }

    pub async fn create(self, create_dto_user: CreateDtoUser) -> impl Responder {
        match self
            .client
            .query(
                CREATE_USER,
                &[
                    &create_dto_user.name,
                    &create_dto_user.email,
                    &create_dto_user.password,
                ],
            )
            .await
        {
            Ok(_) => return "Sucess",
            Err(_) => return "Error",
        };
    }

    pub async fn find_by_id(self, id: i32) -> actix_web::Result<web::Json<FindUserDto>, UserError> {
        match self.client.query(FIND_USER_BY_ID, &[&id]).await {
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
}
