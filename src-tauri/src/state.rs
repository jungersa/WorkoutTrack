use tauri::{AppHandle, State, Manager};
use sqlx::SqlitePool;

pub struct AppState {
    pub db: std::sync::Mutex<Option<SqlitePool>>,
}

pub trait ServiceAccess {
    fn db<F, TResult>(&self, operation: F) -> TResult where F: FnOnce(&SqlitePool) -> TResult;
}

impl ServiceAccess for AppHandle {
    fn db<F, TResult>(&self, operation: F) -> TResult where F: FnOnce(&SqlitePool) -> TResult {
      let app_state: State<AppState> = self.state();
      let db_connection_guard = app_state.db.lock().unwrap();
      let db = db_connection_guard.as_ref().unwrap();

      operation(db)
    }
}