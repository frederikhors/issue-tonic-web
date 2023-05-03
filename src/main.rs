use crate::routes::grpc;
use axum::{error_handling::HandleErrorLayer, routing::get, BoxError};
use std::net::SocketAddr;
use tower::Layer;

pub mod routes;

#[tokio::main]
async fn main() {
    let mut api = axum::Router::new().route("/api", get(|| async { "Hello, World!" }));

    let grpc = HandleErrorLayer::new(|err: BoxError| async move {
        eprintln!("{} {}", err, "grpc service failed");
        [("grpc-status", 13)]
    })
    .layer(grpc::new_grpc_server());

    api = api.nest_service("/grpc", grpc);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    axum::Server::bind(&addr)
        .serve(api.into_make_service())
        .await
        .unwrap();
}
