use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserCreateDto {
    pub name: String,
    pub email: String,
    pub password: String,
}
