use std::io;
use std::time::Duration;
use tokio::time;

use GrpcExample::{ClientRequest, ClientStreamMsg, example_client::ExampleClient};
use tonic::transport::{ClientTlsConfig, Certificate, Channel};

#[allow(non_snake_case)]
pub mod GrpcExample {
  tonic::include_proto!("grpc_example");
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut buffer = String::new();

    //let mut client = ExampleClient::connect("http://localhost:5001").await?;
    let cert = std::fs::read_to_string("server.pem")?;
    let channel = Channel::from_static("https://localhost:5001")
        .tls_config(ClientTlsConfig::new().ca_certificate(Certificate::from_pem(&cert)))?
        .connect()
        .await?;
    
    let mut client = ExampleClient::new(channel);


    let response = client.simple(ClientRequest {message : "World".to_string() }).await?;

    println!("Response: {:?}", response.into_inner());
    println!("Press enter to continue");
    stdin.read_line(&mut buffer)?;

    let client_stream = async_stream::stream! {
      let mut interval = time::interval(Duration::from_secs(1));
      for i in 0..10 {
        interval.tick().await;
        let client_msg = ClientStreamMsg { message : format!("Message {}", i)};
        println!("Sending {:?}", client_msg);
        yield client_msg;
      }
    };

    let response = client.client_stream(client_stream).await?;
    println!("Response: {:?}", response.into_inner());
    println!("=== Client Stream Complete ===");
    println!("Press enter to continue");
    stdin.read_line(&mut buffer)?;


    let response = client.server_stream(ClientRequest { message : "20".to_string()}).await?;
    let mut server_stream = response.into_inner();

    while let Some(server_msg) = server_stream.message().await? {
        println!("Received {:?}", server_msg);
    }
    println!("=== Server Stream Complete ===");
    println!("Press enter to continue");
    stdin.read_line(&mut buffer)?;


    Ok(())
}
