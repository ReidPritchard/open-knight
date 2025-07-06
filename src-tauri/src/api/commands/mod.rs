use crate::db::{connect_db, run_migrations, DatabaseConfig};
use crate::entities::user;
use crate::models::AppUser;
use crate::session::GameSessionManager;
use crate::utils::AppError;
use log::warn;
use ok_engine_manager::events::EventEmitter;
use ok_engine_manager::manager::EngineManager;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait};
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::sync::Mutex;

pub mod chess;
pub mod database;
pub mod engine;
pub mod session;

#[derive(Clone)]
pub struct AppHandleEmitter {
    app_handle: Arc<AppHandle>,
}

impl EventEmitter for AppHandleEmitter {
    fn emit_event(
        &self,
        event: &str,
        data: std::string::String,
    ) {
        let _ = self.app_handle.emit(event, data);
    }
}

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
    pub user: AppUser,

    // Managers
    pub engine_manager: Mutex<EngineManager<AppHandleEmitter>>,
    pub game_session_manager: Mutex<GameSessionManager>,
}

impl AppState {
    /// Creates a new AppState instance with initialized database connection and engine manager
    pub async fn new(app_handle: AppHandle) -> Result<Self, AppError> {
        let database_url = crate::db::resolve_database_url(&app_handle).await;
        let migration_config = DatabaseConfig {
            max_connections: 1,
            min_connections: 1,
            url: database_url.clone(),
        };
        let migration_db = connect_db(migration_config).await?;

        run_migrations(&migration_db).await?;

        let app_db_config = DatabaseConfig {
            max_connections: 5,
            min_connections: 1,
            url: database_url,
        };
        let db = connect_db(app_db_config).await?;

        // Get user from database or create default
        let user = match user::Entity::find().one(&db).await.map_err(|e| {
            AppError::DatabaseError(format!("Failed to query user: {}", e))
        })? {
            Some(existing_user) => existing_user,
            None => {
                warn!("No user found, creating default user");
                // Create and insert default user
                let default_user = AppUser::default();
                let default_user_active: user::ActiveModel =
                    default_user.into();
                default_user_active.insert(&db).await.map_err(|e| {
                    AppError::DatabaseError(format!(
                        "Failed to create default user: {}",
                        e
                    ))
                })?
            }
        };

        let app_handle_reference = Arc::new(app_handle);
        let event_emitter = Arc::new(AppHandleEmitter {
            app_handle: app_handle_reference.clone(),
        });

        Ok(Self {
            app_handle: app_handle_reference,
            db,
            user: user.into(),
            engine_manager: Mutex::new(EngineManager::with_emitter(
                event_emitter,
            )),
            game_session_manager: Mutex::new(GameSessionManager::new()),
        })
    }
}
