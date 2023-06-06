pub const CREATE_USER: &'static str = "INSERT INTO users VALUES (DEFAULT, $1, $2, $3)";
pub const FIND_USER_BY_ID: &'static str = "SELECT * FROM users WHERE id=$1";
