use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use crate::{app_state::AppState, utils::auth::validate_token, AuthAPIError};

pub async fn verify_token(
    State(_state): State<AppState>,
    Json(request): Json<VerifyTokenRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let token = request.token;
    let _claims = validate_token(&token)
        .await
        .map_err(|_| AuthAPIError::InvalidToken)?;

    Ok(StatusCode::OK.into_response())
}

#[derive(Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}
