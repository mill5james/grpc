use std::time::Duration;
use std::{fs, io};
use tokio::sync::mpsc;
use tokio::time;
use tonic::transport::{Certificate, Channel, ClientTlsConfig};
use tonic::Response;

pub mod grpc_example {
    tonic::include_proto!("grpc_example");
}

use grpc_example::{example_client::*, *};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut buffer = String::new();

    //let mut client = ExampleClient::connect("http://localhost:5001").await?;
    let cert = match fs::read_to_string("certificate.pem") {
        Ok(value) => value,
        Err(_) => panic!("Could not read the certificate.pem file - Please look at the readme.md"),
    };
    let channel = Channel::from_static("https://localhost:5001")
        .tls_config(ClientTlsConfig::new().ca_certificate(Certificate::from_pem(&cert)))?
        .connect()
        .await?;

    let mut client = ExampleClient::new(channel);

    let response: Response<ServerResponse> = client
        .simple(ClientRequest {
            message: "World".to_string(),
        })
        .await?;
    let server_response = response.into_inner();

    println!("Response: {:?}", server_response);
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

    let response = client
        .server_stream(ClientRequest {
            message: "10".to_string(),
        })
        .await?;
    let mut server_stream = response.into_inner();

    while let Some(server_msg) = server_stream.message().await? {
        println!("Received {:?}", server_msg);
    }
    println!("=== Server Stream Complete ===");
    println!("Press enter to continue");
    stdin.read_line(&mut buffer)?;

    let (tx, mut rx) = mpsc::channel(4);
    let up_stream = async_stream::stream! {
      let mut interval = time::interval(Duration::from_secs(1));
      let mut i = 0;
      loop {
        interval.tick().await;
        let client_msg = ClientStreamMsg { message : format!("{}", i)};
        println!("Sending {:?}", client_msg);
        yield client_msg;
        let value = rx.recv().await.unwrap();
        i = value + 1;
      }
    };

    let response = client.bi_dir_stream(up_stream).await?;
    let mut server_stream = response.into_inner();
    while let Some(server_msg) = server_stream.message().await? {
        println!("Received {:?}", server_msg);
        let i = match server_msg.message.parse::<i32>() {
            Ok(n) => n,
            Err(_) => i32::MIN,
        };
        tx.send(i).await.unwrap();
        if i > 20 {
            break;
        }
    }

    Ok(())
}
