use crate::routes::grpc;
use axum::routing::get;
use std::net::SocketAddr;

pub mod routes;

#[tokio::main]
async fn main() {
    let mut api = axum::Router::new().route("/api", get(|| async { "Hello, World!" }));

    let grpc_server = grpc::new_grpc_server();

    api = api.route(
        "/*rpc",
        // I don't know what to use here
        axum::routing::any_service(grpc_server.into_service()),
        // get(grpc_server.into_service())
    );

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    axum::Server::bind(&addr)
        .serve(api.into_make_service())
        .await
        .unwrap();
}
