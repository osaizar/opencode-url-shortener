use std::sync::Arc;

use axum::extract::{rejection::JsonRejection, Path, State};
use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::code::CodeGenerator;
use crate::error::{ApiError, AppResult};
use crate::store::Store;
use crate::validation::validate_target;

pub const MAX_CODE_ATTEMPTS: usize = 10;

#[derive(Clone)]
pub struct AppState {
    pub store: Store,
    pub public_base_url: String,
    pub code_generator: Arc<dyn CodeGenerator>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUrlRequest {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CreateUrlResponse {
    pub code: String,
    pub short_url: String,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct HealthResponse {
    pub status: &'static str,
}

pub async fn create_url(
    State(state): State<AppState>,
    payload: Result<Json<CreateUrlRequest>, JsonRejection>,
) -> AppResult<(StatusCode, Json<CreateUrlResponse>)> {
    let Json(request) = payload.map_err(|_| ApiError::InvalidRequest)?;
    validate_target(&request.url)?;

    for _ in 0..MAX_CODE_ATTEMPTS {
        let code = state.code_generator.generate();
        if state.store.insert(&code, &request.url).await? {
            let base = state.public_base_url.trim_end_matches('/');
            return Ok((
                StatusCode::CREATED,
                Json(CreateUrlResponse {
                    code: code.clone(),
                    short_url: format!("{base}/{code}"),
                }),
            ));
        }
    }

    Err(ApiError::Internal)
}

pub async fn redirect(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> AppResult<impl IntoResponse> {
    match state.store.find(&code).await? {
        Some(target) => Ok((StatusCode::FOUND, [(header::LOCATION, target)])),
        None => Err(ApiError::NotFound),
    }
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}
