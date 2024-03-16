use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use serde_json::{json, Value};

use crate::{model::Document, state::RasteState, storage::Storage};

pub async fn post_doc<S: Storage + Clone>(
    State(state): State<RasteState<S>>,
    body: String,
) -> Json<Value> {
    let key = state.storage.write_doc(&body);
    Json(json!({
        "key": key
    }))
}

pub async fn get_doc<S: Storage + Clone>(
    State(state): State<RasteState<S>>,
    Path(key): Path<String>,
) -> Response {
    match state.storage.read_doc(&key) {
        Some(data) => Json(Document { key, data }).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(json!({ "message": "Document not found." })),
        )
            .into_response(),
    }
}
