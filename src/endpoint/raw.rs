use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::{state::RasteState, storage::Storage};

pub async fn get_raw<S: Storage + Clone>(
    State(state): State<RasteState<S>>,
    Path(key): Path<String>,
) -> Response {
    match state.storage.read_doc(&key) {
        Some(data) => ([(header::CONTENT_TYPE, "text/plain; charset=UTF-8")], data).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(json!({ "message": "Document not found." })),
        )
            .into_response(),
    }
}
