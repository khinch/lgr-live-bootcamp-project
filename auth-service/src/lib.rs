use askama::Template;
use axum::{
    http::{Method, StatusCode},
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    serve::Serve,
    Json, Router,
};

use redis::{Client, RedisResult};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::error::Error;
use tokio::signal;
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};

use domain::AuthAPIError;
pub mod routes;
use crate::routes::{
    delete_user, login, logout, signup, verify_2fa, verify_token,
};
use crate::utils::{constants::APP_SERVICE_EXTERNAL_ADDRESS, tracing::*};
pub mod app_state;
pub mod domain;
pub mod services;
use app_state::AppState;
pub mod utils;

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists => {
                (StatusCode::CONFLICT, "User already exists")
            }
            AuthAPIError::ValidationError => {
                (StatusCode::BAD_REQUEST, "Invalid input")
            }
            AuthAPIError::UserNotFound => {
                (StatusCode::NOT_FOUND, "User not found")
            }
            AuthAPIError::IncorrectCredentials => {
                (StatusCode::UNAUTHORIZED, "Incorrect credentials")
            }
            AuthAPIError::UnexpectedError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
            }
            AuthAPIError::MissingToken => {
                (StatusCode::BAD_REQUEST, "Missing token")
            }
            AuthAPIError::InvalidToken => {
                (StatusCode::UNAUTHORIZED, "Invalid token")
            }
        };
        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });
        (status, body).into_response()
    }
}

pub struct Application {
    server: Serve<Router, Router>,
    pub address: String,
}

impl Application {
    pub async fn build(
        app_state: AppState,
        address: &str,
    ) -> Result<Self, Box<dyn Error>> {
        let allowed_origins = [
            "http://localhost:8000".parse()?,
            "http://localhost:5000".parse()?,
            "http://127.0.0.1:8000".parse()?,
            "http://127.0.0.1:5000".parse()?,
            "http://app-service:8000".parse()?,
            "http://67.205.162.100:8000".parse()?,
            "https://67.205.162.100:8000".parse()?,
            "http://lgr.testwebsitepleaseignore.uk:8000".parse()?,
            "https://lgr.testwebsitepleaseignore.uk:8000".parse()?,
        ];

        let cors = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST])
            .allow_credentials(true)
            .allow_origin(allowed_origins);

        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/verify-2fa", post(verify_2fa))
            .route("/logout", post(logout))
            .route("/verify-token", post(verify_token))
            .route("/delete-user", delete(delete_user))
            .route("/app.js", get(serve_app_js))
            .with_state(app_state)
            .layer(cors)
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(make_span_with_request_id)
                    .on_request(on_request)
                    .on_response(on_response),
            );

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        Ok(Application { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        tracing::info!("listening on {}", &self.address);
        self.server.with_graceful_shutdown(shutdown_signal()).await
    }
}

#[allow(dead_code)]
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[allow(dead_code)]
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

#[derive(Template)]
#[template(path = "app.js", escape = "none")]
struct AppJsTemplate {
    app_service_external_address: String,
}

async fn serve_app_js() -> impl IntoResponse {
    let app_service_external_address = APP_SERVICE_EXTERNAL_ADDRESS.to_string();

    let template = AppJsTemplate {
        app_service_external_address,
    };

    (
        [("content-type", "application/javascript")],
        template
            .render()
            .expect("Failed to render auth-service app.js template"),
    )
}

pub async fn get_postgres_pool(url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new().max_connections(5).connect(url).await
}

pub fn get_redis_client(redis_hostname: String) -> RedisResult<Client> {
    let redis_url = format!("redis://{}/", redis_hostname);
    redis::Client::open(redis_url)
}
