use sqlx::types::Uuid;

use crate::entities::User;
use crate::models::UserDTO;

impl From<User> for UserDTO {
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_string(),
            username: user.username,
            admin: user.admin,
            added_datetime: user.added_datetime,
            updated_datetime: user.updated_datetime,
        }
    }
}

impl From<UserDTO> for User {
    fn from(user: UserDTO) -> Self {
        Self {
            id: Uuid::parse_str(&user.id).expect("Id was not valid Uuid"),
            username: user.username,
            password: String::default(),
            admin: user.admin,
            added_datetime: user.added_datetime,
            updated_datetime: user.updated_datetime,
        }
    }
}
