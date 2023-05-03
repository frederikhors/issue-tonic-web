use super::gen::{
    hello::{
        hello_greeter_server::{HelloGreeter, HelloGreeterServer},
        HelloReply, HelloRequest,
    },
    world::{
        world_greeter_server::{WorldGreeter, WorldGreeterServer},
        WorldReply, WorldRequest,
    },
};
use tonic::{transport::server::Router, Response, Status};

#[derive(Default)]
struct HelloServiceImpl {}

#[tonic::async_trait]
impl HelloGreeter for HelloServiceImpl {
    async fn say_hello(
        &self,
        request: tonic::Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

#[derive(Default)]
struct WorldServiceImpl {}

#[tonic::async_trait]
impl WorldGreeter for WorldServiceImpl {
    async fn say_world(
        &self,
        request: tonic::Request<WorldRequest>,
    ) -> Result<Response<WorldReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = WorldReply {
            message: format!("World {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

pub fn new_grpc_server() -> Router {
    // I need to expose these services as grpc-web (using tonic_web I think)

    // let server = tonic_web::enable(GreeterServer::new(GrpcServiceImpl::default()));

    let server = tonic::transport::Server::builder()
        .add_service(HelloGreeterServer::new(HelloServiceImpl::default()))
        .add_service(WorldGreeterServer::new(WorldServiceImpl::default()));

    server
}
