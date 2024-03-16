use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use crate::{state::RasteState, storage::Storage};

pub async fn head_status<S: Storage + Clone>(
    State(state): State<RasteState<S>>,
    Path(key): Path<String>,
) -> StatusCode {
    match state.storage.read_doc(&key) {
        Some(_) => StatusCode::OK,
        None => StatusCode::NOT_FOUND,
    }
}
