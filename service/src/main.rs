use tonic::{transport::Server, Request, Response, Status};

use statedb_service::state_db_service_server::{StateDbService, StateDbServiceServer};
use statedb_service::{
    FlushResponse, GetProgramRequest, GetProgramResponse, GetRequest, GetResponse, LoadDbRequest,
    LoadProgramDbRequest, SetProgramRequest, SetProgramResponse, SetRequest, SetResponse,
};

pub mod statedb_service {
    tonic::include_proto!("statedb.v1"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct StateDBServiceSVC {}

#[tonic::async_trait]
impl StateDbService for StateDBServiceSVC {
    async fn get(
        &self,
        request: Request<GetRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<GetResponse>, Status> {
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let reply = GetResponse::default();

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn set(
        &self,
        request: Request<SetRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<SetResponse>, Status> {
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let reply = SetResponse::default();

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn set_program(
        &self,
        request: Request<SetProgramRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<SetProgramResponse>, Status> {
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let reply = SetProgramResponse::default();

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn get_program(
        &self,
        request: Request<GetProgramRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<GetProgramResponse>, Status> {
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let reply = GetProgramResponse::default();

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn load_db(
        &self,
        request: Request<LoadDbRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<()>, Status> {
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        Ok(Response::new(())) // Send back our formatted greeting
    }

    async fn load_program_db(
        &self,
        request: Request<LoadProgramDbRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<()>, Status> {
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        Ok(Response::new(())) // Send back our formatted greeting
    }

    async fn flush(
        &self,
        request: Request<()>, // Accept request of type HelloRequest
    ) -> Result<Response<FlushResponse>, Status> {
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let reply = FlushResponse::default();

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = StateDBServiceSVC::default();

    Server::builder()
        .add_service(StateDbServiceServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
