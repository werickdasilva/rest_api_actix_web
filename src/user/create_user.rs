use super::create_user_dto::UserCreateDto;

pub fn create_user(create_user_dto: UserCreateDto) {
    println!("Create User: {:#?}", create_user_dto);
}
