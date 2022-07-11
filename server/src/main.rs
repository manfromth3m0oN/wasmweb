use std::net::SocketAddr;

use axum::{extract::Path, response::IntoResponse, routing::get, Router};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/:binary", get(binary));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> impl IntoResponse {
    "This API has no root. use /:name-of-bin to get a binary"
}

async fn binary(Path(binary_name): Path<String>) -> impl IntoResponse {
    match std::fs::read(format!("./bins/{}", binary_name)) {
        Ok(data) => return data,
        Err(e) => {
            tracing::error!("No binary named {}", binary_name);
            return vec![];
        }
    }
}
