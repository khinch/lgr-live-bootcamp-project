use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, UserStoreError},
};

pub async fn delete_user(
    State(state): State<AppState>,
    Json(request): Json<DeleteUserRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::parse(request.email).map_err(|_| AuthAPIError::ValidationError)?;

    {
        let mut user_store = state.user_store.write().await;
        user_store.delete_user(&email).await.map_err(|e| match e {
            UserStoreError::UserNotFound => AuthAPIError::UserNotFound,
            _ => AuthAPIError::UnexpectedError,
        })?;
    }

    let message = format!("User deleted: {}", email.as_ref());
    let response = Json(DeleteUserResponse { message: message });

    Ok((StatusCode::OK, response))
}

#[derive(Deserialize)]
pub struct DeleteUserRequest {
    pub email: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct DeleteUserResponse {
    pub message: String,
}
