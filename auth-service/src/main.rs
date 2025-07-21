use auth_service::Application;

#[tokio::main]
async fn main() {
    let application = Application::build("0.0.0.0:3000")
        .await
        .expect("Failed to build auth-service application");
    application
        .run()
        .await
        .expect("Failed to run auth-service application");
}
