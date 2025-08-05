use std::sync::Arc;
use tokio::sync::RwLock;

use auth_service::{
    app_state::AppState, services::HashmapUserStore, utils::constants::prod, Application,
};

#[tokio::main]
async fn main() {
    let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
    let app_state = AppState::new(user_store);

    let application = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build auth-service application");
    application
        .run()
        .await
        .expect("Failed to run auth-service application");
}
