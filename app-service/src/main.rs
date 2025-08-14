use std::env;

use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use axum_extra::extract::CookieJar;
use serde::Serialize;
use tokio::signal;
use tower_http::services::ServeDir;

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

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/", get(root))
        .route("/protected", get(protected));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    login_link: String,
    logout_link: String,
}

async fn root() -> impl IntoResponse {
    let default_address = String::from("http://localhost:3000");
    let auth_service_external_address = match env::var("AUTH_SERVICE_EXTERNAL_ADDRESS") {
        Ok(address) => {
            if address.is_empty() {
                default_address
            } else {
                address
            }
        }
        Err(_) => default_address,
    };

    let login_link = format!("{}", auth_service_external_address);
    let logout_link = format!("{}/logout", auth_service_external_address);

    let template = IndexTemplate {
        login_link,
        logout_link,
    };
    Html(template.render().unwrap())
}

async fn protected(jar: CookieJar) -> impl IntoResponse {
    let jwt_cookie = match jar.get("jwt") {
        Some(cookie) => cookie,
        None => {
            return StatusCode::UNAUTHORIZED.into_response();
        }
    };

    let api_client = reqwest::Client::builder().build().unwrap();

    let verify_token_body = serde_json::json!({
        "token": &jwt_cookie.value(),
    });

    let default_address = String::from("http://localhost:3000");
    let auth_service_container_address = match env::var("AUTH_SERVICE_CONTAINER_ADDRESS") {
        Ok(address) => {
            if address.is_empty() {
                default_address
            } else {
                address
            }
        }
        Err(_) => default_address,
    };

    let url = format!("{}/verify-token", auth_service_container_address);

    let response = match api_client.post(&url).json(&verify_token_body).send().await {
        Ok(response) => response,
        Err(_) => {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    match response.status() {
        reqwest::StatusCode::UNAUTHORIZED | reqwest::StatusCode::BAD_REQUEST => {
            StatusCode::UNAUTHORIZED.into_response()
        }
        reqwest::StatusCode::OK => Json(ProtectedRouteResponse {
            img_url: "https://i.ibb.co/YP90j68/Light-Live-Bootcamp-Certificate.png".to_owned(),
        })
        .into_response(),
        _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[derive(Serialize)]
pub struct ProtectedRouteResponse {
    pub img_url: String,
}
