use tonic::{transport::Server, Request, Response, Status};
use tikv_client::{Config, IntoOwnedRange, Key, KvPair, RawClient, Value};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

pub mod hello_world {
    tonic::include_proto!("helloworld"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(&self, request: Request<HelloRequest>
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        let config = Config::default();
        let client = RawClient::new_with_config(vec!["n1:2379"], config).await.unwrap();
        client.put("key".to_owned(), "value".to_owned()).await;
        let value = client.get("key".to_owned()).await;
        println!("value: {:?}", value);

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
    .add_service(GreeterServer::new(greeter))
    .serve(addr)
    .await?;
    
    Ok(())
}