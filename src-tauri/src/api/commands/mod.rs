use crate::db::{connect_db, run_migrations};
use crate::engine::manager::EngineManager;
use crate::entities::user;
use crate::session::GameSessionManager;
use crate::utils::AppError;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use std::sync::Arc;
use tauri::AppHandle;
use tokio::sync::Mutex;

pub mod chess;
pub mod database;
pub mod engine;
pub mod game;

/// Application state shared across Tauri commands
///
/// Contains:
/// - The Tauri application handle
/// - Database connection
/// - Engine manager to interact with chess engines
pub struct AppState {
    pub app_handle: Arc<AppHandle>,
    pub db: DatabaseConnection,

    // User
    pub user: user::Model,

    // Managers
    pub engine_manager: Mutex<EngineManager>,
    pub game_session_manager: Mutex<GameSessionManager>,
}

impl AppState {
    /// Creates a new AppState instance with initialized database connection and engine manager
    pub async fn new(app_handle: AppHandle) -> Result<Self, AppError> {
        let db = connect_db().await?;
        run_migrations(&db).await?;

        // Get user from database or create default
        let user = match user::Entity::find()
            .one(&db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to query user: {}", e)))?
        {
            Some(existing_user) => existing_user,
            None => {
                println!("No user found, creating default user");
                // Create and insert default user
                let default_user = user::Model::default();
                let default_user_active = user::ActiveModel {
                    username: Set(default_user.username),
                    email: Set(default_user.email),
                    user_id: Set(default_user.user_id),
                    created_at: Set(default_user.created_at),
                };
                default_user_active.insert(&db).await.map_err(|e| {
                    AppError::DatabaseError(format!("Failed to create default user: {}", e))
                })?
            }
        };

        Ok(Self {
            app_handle: Arc::new(app_handle),
            db,
            user,
            engine_manager: Mutex::new(EngineManager::new()),
            game_session_manager: Mutex::new(GameSessionManager::new()),
        })
    }
}
