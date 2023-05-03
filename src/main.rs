use crate::routes::grpc;
use axum::{error_handling::HandleErrorLayer, routing::get, BoxError};
use std::net::SocketAddr;
use tower::Layer;

pub mod routes;

#[tokio::main]
async fn main() {
    let mut api = axum::Router::new().route("/api", get(|| async { "Hello, World!" }));

    let grpc_server = grpc::new_grpc_server();

    // Is it possible to move this func in grpc::new_grpc_server() ?
    let grpc = HandleErrorLayer::new(|err: BoxError| async move {
        eprintln!("{} {}", err, "grpc service failed");
        // from https://docs.rs/tonic/latest/src/tonic/status.rs.html#100
        let internal_error_code = 13;
        [("grpc-status", internal_error_code)]
    })
    .layer(grpc_server.into_service());

    api = api.nest_service("/grpc", grpc);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    axum::Server::bind(&addr)
        .serve(api.into_make_service())
        .await
        .unwrap();
}
