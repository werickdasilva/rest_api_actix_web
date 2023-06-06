use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateDtoUser {
    pub name: String,
    pub email: String,
    pub password: String,
}
