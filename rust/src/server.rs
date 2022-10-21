use std::time::Duration;
use std::fs;
use log::LevelFilter;
use futures::StreamExt;
use tokio::sync::mpsc;
use tokio::time;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Status, Request, Response, Streaming, transport::{Server, Identity, ServerTlsConfig}};

pub mod grpc_example {
  tonic::include_proto!("grpc_example");
}
use grpc_example::{ClientRequest, ClientStreamMsg, ServerResponse, ServerStreamMsg, example_server::{Example, ExampleServer}};


#[derive(Debug, Default)]
pub struct ExampleService {
}

#[tonic::async_trait]
impl Example for ExampleService {
        async fn simple(
            &self,
            request: Request<ClientRequest>,
        ) -> Result<Response<ServerResponse>, Status> {
          let client_req = request.into_inner();
          Ok(Response::new(ServerResponse { message: format!("Hello {}", client_req.message )}))
        }

        async fn client_stream(
            &self,
            request: Request<Streaming<ClientStreamMsg>>,
        ) -> Result<Response<ServerResponse>, Status> {
          let mut stream = request.into_inner();
          let mut count = 0;

          while let Some(client_msg) = stream.next().await {
            let client_msg = client_msg?;
            log::info!("Received {:?}", client_msg);
            count += 1;

          }

          Ok(Response::new(ServerResponse { message: count.to_string() }))
        }

        type ServerStreamStream = ReceiverStream<Result<ServerStreamMsg, Status>>;
        async fn server_stream(
          &self,
          request: Request<ClientRequest>,
        ) -> Result<Response<Self::ServerStreamStream>, Status> {
          let client_req = request.into_inner();
          let send_count: i32;
          match client_req.message.parse::<i32>() {
            Ok(n) => send_count = n,
            Err(_e) => send_count = 10,
          }
          log::info!("Attempting to send {} messages", send_count);

          let (tx, rx) = mpsc::channel(4);
          tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(1));
            for i in 0..send_count {
              let server_msg = ServerStreamMsg{ message : format!("Message {}", i)};
              log::info!("Sending {:?}", server_msg);
              tx.send(Ok(server_msg)).await.unwrap();
              interval.tick().await;
            }
          });
          Ok(Response::new(ReceiverStream::new(rx)))

        }

        type BiDirStreamStream: = ReceiverStream<Result<ServerStreamMsg, Status>>;

        async fn bi_dir_stream(
          &self,
          _request: Request<Streaming<ClientStreamMsg>>,
        ) -> Result<Response<Self::BiDirStreamStream>, Status> {
          unimplemented!()
        }

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let cert = match fs::read_to_string("certificate.pem") {
      Ok(value) => value,
      Err(_) => panic!("Could not read the certificate.pem file - Please look at the readme.md")
    };

    let key = match fs::read_to_string("certificate.key") {
      Ok(value) => value,
      Err(_) => panic!("Could not read the certificate.key file - Please look at the readme.md")
    };

    let identity = Identity::from_pem(cert, key);
    let addr = "[::1]:5001".parse()?;
    let service = ExampleService::default();

    Server::builder()
        .tls_config(ServerTlsConfig::new().identity(identity))?
        .add_service(ExampleServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
