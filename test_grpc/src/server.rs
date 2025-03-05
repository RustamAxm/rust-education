use helloworld::greeter_server::{Greeter, GreeterServer};
use helloworld::{HelloReply, HelloRequest};
use tonic::{transport::Server, Request, Response, Status};
mod helloworld;

// defining a struct for our service
#[derive(Default)]
pub struct MyGreeter {}

// implementing rpc for service defined in .proto
#[tonic::async_trait]
impl Greeter for MyGreeter {
    // our rpc impelemented as function
    async fn say_hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloReply>, Status> {
        // returning a response as SayResponse message as defined in .proto
        let vec = vec![1.2, 2.4]; 
        println!("client {}", request.get_ref().name);
        println!("client vec = {} {}", request.get_ref().payload[0], request.get_ref().payload[1]);
        Ok(Response::new(HelloReply {
            // reading data from request which is awrapper around our SayRequest message defined in .proto
            message: format!("hello {}", request.get_ref().name),
            id: 23,
            payload: vec,
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // defining address for our service
    let addr = "[::1]:50051".parse().unwrap();
    // creating a service
    let say = MyGreeter::default();
    println!("Server listening on {}", addr);
    // adding our service to our server.
    Server::builder()
        .add_service(GreeterServer::new(say))
        .serve(addr)
        .await?;
    Ok(())
}
