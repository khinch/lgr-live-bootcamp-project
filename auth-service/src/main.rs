use std::sync::Arc;
use tokio::sync::RwLock;

use auth_service::{app_state::AppState, services::HashmapUserStore, Application};

#[tokio::main]
async fn main() {
    let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
    let app_state = AppState::new(user_store);

    let application = Application::build(app_state, "0.0.0.0:3000")
        .await
        .expect("Failed to build auth-service application");
    application
        .run()
        .await
        .expect("Failed to run auth-service application");
}
