use serde::Serialize;

use crate::domain::role::Role;
use crate::domain::user::User;

#[derive(Serialize)]
pub struct UserDto {
    pub id: String,
    pub username: String,
    pub email: String,
    pub name: Option<String>,
    pub lastname: Option<String>,
    pub validated: bool,
    pub role: String,
}

impl From<&User> for UserDto {
    fn from(user: &User) -> Self {
        UserDto {
            id: user.base().id().to_string(),
            username: user.identity().username().to_string(),
            email: user.identity().email().to_string(),
            name: user.person().map(|p| p.fullname().name().to_string()),
            lastname: user.person().map(|p| p.fullname().lastname().to_string()),
            validated: user.is_validated(),
            role: user.role().base().id().to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct RoleDto {
    pub id: String,
    pub name: String,
}

impl From<&Role> for RoleDto {
    fn from(role: &Role) -> Self {
        RoleDto {
            id: role.base().id().to_string(),
            name: role.name().to_string(),
        }
    }
}
