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
use tonic::{transport::server::Routes, Response, Status};

#[derive(Default)]
pub struct HelloServiceImpl {}

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
pub struct WorldServiceImpl {}

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

pub fn new_grpc_server() -> Routes {
    tonic::transport::Server::builder()
        .add_service(tonic_web::enable(HelloGreeterServer::new(
            HelloServiceImpl::default(),
        )))
        .add_service(tonic_web::enable(WorldGreeterServer::new(
            WorldServiceImpl::default(),
        )))
        .into_service()
}
