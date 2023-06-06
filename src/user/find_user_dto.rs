use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FindUserDto {
    id: i32,
    name: String,
    email: String,
}

impl From<Row> for FindUserDto {
    fn from(row: Row) -> Self {
        Self {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
        }
    }
}
