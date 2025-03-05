use helloworld::greeter_client::GreeterClient;
use helloworld::HelloRequest;

mod helloworld;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // creating a channel ie connection to server
    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
        .connect()
        .await?;
    // creating gRPC client from channel
    let mut client = GreeterClient::new(channel);
    // creating a new Request
    let vec = vec![3.4, 5.6];
    let request = tonic::Request::new(HelloRequest {
        name: String::from("client data"),
        id: 2,
        payload: vec,
    });
    // sending request and waiting for response
    let response = client.say_hello(request).await?.into_inner();
    println!("RESPONSE={:?}", response);
    Ok(())
}
