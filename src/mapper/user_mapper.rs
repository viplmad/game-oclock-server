use crate::entities::User;
use crate::models::UserDTO;

impl From<User> for UserDTO {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            added_datetime: user.added_datetime,
            updated_datetime: user.updated_datetime,
        }
    }
}

impl From<UserDTO> for User {
    fn from(user: UserDTO) -> Self {
        Self {
            id: user.id,
            username: user.username,
            password: String::default(),
            added_datetime: user.added_datetime,
            updated_datetime: user.updated_datetime,
        }
    }
}
