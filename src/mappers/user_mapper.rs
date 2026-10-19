use uuid::Uuid;

use crate::entities::User;
use crate::models::UserDTO;

impl From<User> for UserDTO {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            role: user.role,
            added_datetime: user.added_datetime,
            updated_datetime: user.updated_datetime,
        }
    }
}

impl From<UserDTO> for User {
    fn from(user: UserDTO) -> Self {
        Self {
            id: Uuid::default(),
            username: user.username,
            password: String::default(),
            role: user.role,
            added_datetime: user.added_datetime,
            updated_datetime: user.updated_datetime,
        }
    }
}
