use axum::{
    routing::{get, post},
    Router,
};

use state::RasteState;
use std::path::Path as fsPath;
use tower_http::services::{ServeDir, ServeFile};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    endpoint::{
        documents::{get_doc, post_doc},
        raw::get_raw,
        status::head_status,
    },
    storage::DBEngine,
};

mod endpoint;
mod keygen;
mod model;
mod state;
mod storage;

type State = RasteState<DBEngine>;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "raste=debug,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_path = fsPath::new("data");
    let state = State::new(db_path);

    // TODO: pluggable frontend
    let serve_dir = ServeDir::new("dist").not_found_service(ServeFile::new("dist/index.html"));
    let app = Router::new()
        .route("/raw/:id", get(get_raw).head(head_status))
        .route("/documents/:id", get(get_doc).head(head_status))
        .route("/documents", post(post_doc))
        .fallback_service(serve_dir)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8290").await.unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
