use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserCreateDto {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub password: String,
}
