use axum::{extract, routing::post, Json, Router};
use tower_http::cors::{Any, CorsLayer};

use std::net::SocketAddr;

use wordle_assistant::recommendation_api::{get_recommendations, State, WordleResponse};

async fn handle(extract::Json(payload): extract::Json<State>) -> Json<WordleResponse> {
    Json(get_recommendations(payload))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/recommendations", post(handle)).layer(
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any),
    );
    let addr = SocketAddr::from(([127, 0, 0, 1], 5000));
    println!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
