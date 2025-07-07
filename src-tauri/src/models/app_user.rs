use crate::entities::user;
use ok_utils::ts_export;
use sea_orm::ActiveValue::Set;

/// A user of the application. Wraps the user db entity.
#[ts_export]
#[derive(Default)]
pub struct AppUser {
    pub user_id: Option<i32>,
    pub username: String,
    pub email: String,
    pub created_at: Option<String>,
}

/// Convert from an AppUser to a user::Model
impl From<AppUser> for user::Model {
    fn from(app_user: AppUser) -> Self {
        user::Model {
            user_id: app_user.user_id.unwrap_or(0),
            username: app_user.username,
            email: app_user.email,
            created_at: app_user.created_at.map(|s| s.parse().unwrap()),
        }
    }
}

/// Convert from an AppUser to a user::ActiveModel
impl From<AppUser> for user::ActiveModel {
    fn from(app_user: AppUser) -> Self {
        user::ActiveModel {
            user_id: Set(app_user.user_id.unwrap_or(0)),
            username: Set(app_user.username),
            email: Set(app_user.email),
            created_at: Set(app_user.created_at.map(|s| s.parse().unwrap())),
        }
    }
}

/// Convert from a user::Model to an AppUser
impl From<user::Model> for AppUser {
    fn from(user: user::Model) -> Self {
        AppUser {
            user_id: Some(user.user_id),
            username: user.username,
            email: user.email,
            created_at: user.created_at.map(|d| d.to_rfc3339()),
        }
    }
}
