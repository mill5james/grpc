use futures::StreamExt;
use log::LevelFilter;
use std::fs;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::signal;
use tokio::sync::mpsc;
use tokio::time;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{
    transport::{Identity, Server, ServerTlsConfig},
    Request, Response, Status, Streaming,
};

pub mod grpc_example {
    tonic::include_proto!("grpc_example");
}
use grpc_example::{
    example_server::{Example, ExampleServer},
    ClientRequest, ClientStreamMsg, ServerResponse, ServerStreamMsg,
};

#[derive(Debug, Default)]
pub struct ExampleService {
    history: Arc<Mutex<Vec<String>>>,
}

impl ExampleService {
    fn new() -> Self {
        Self {
            history: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn dump_db(&self) {
        let values = self.history.lock().unwrap();
        log::info!("history.len()={}", values.len());
        for i in 0..values.len() {
            log::info!("history[{}]={}", i, values.get(i).unwrap());
        }
    }
}

#[tonic::async_trait]
impl Example for ExampleService {
    async fn simple(
        &self,
        request: Request<ClientRequest>,
    ) -> Result<Response<ServerResponse>, Status> {
        let client_req = request.into_inner();
        Ok(Response::new(ServerResponse {
            message: format!("Hello {}", client_req.message),
        }))
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
            let mut vec = self.history.lock().unwrap();
            vec.push(format!("{:?}", client_msg));
        }

        Ok(Response::new(ServerResponse {
            message: count.to_string(),
        }))
    }

    type ServerStreamStream = ReceiverStream<Result<ServerStreamMsg, Status>>;
    async fn server_stream(
        &self,
        request: Request<ClientRequest>,
    ) -> Result<Response<Self::ServerStreamStream>, Status> {
        let client_req = request.into_inner();
        let send_count = match client_req.message.parse::<i32>() {
            Ok(n) => n,
            Err(_e) => 10,
        };

        log::info!("Attempting to send {} messages", send_count);

        let (tx, rx) = mpsc::channel(4);
        let history = self.history.clone();
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(1));
            for i in 0..send_count {
                let server_msg = ServerStreamMsg {
                    message: format!("Message {}", i),
                };
                {
                    let mut vec = history.lock().unwrap();
                    vec.push(format!("{:?}", server_msg));
                }

                log::info!("Sending {:?}", server_msg);
                tx.send(Ok(server_msg)).await.unwrap();
                interval.tick().await;
            }
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    type BiDirStreamStream = ReceiverStream<Result<ServerStreamMsg, Status>>;

    async fn bi_dir_stream(
        &self,
        request: Request<Streaming<ClientStreamMsg>>,
    ) -> Result<Response<Self::BiDirStreamStream>, Status> {
        self.dump_db();

        let mut stream = request.into_inner();
        let (result_tx, result_rx) = mpsc::channel(4);

        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(1));
            while let Some(client_msg) = stream.next().await {
                let msg: ClientStreamMsg = client_msg.unwrap(); 
                log::info!("Received {:?}", msg);

                let value: i32 = match msg.message.parse::<i32>() {
                    Ok(n) => n,
                    Err(_) => i32::MIN,
                };
                let server_msg = ServerStreamMsg {
                    message: format!("{}", value + 1),
                };

                log::info!("Sending {:?}", server_msg);
                result_tx.send(Ok(server_msg)).await.unwrap();
                interval.tick().await;
            }
            //It is important _NOT_ to await the close here since it prevents cleanup
            let _ = result_tx.closed();
        });

        Ok(Response::new(ReceiverStream::new(result_rx)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let cert = match fs::read_to_string("certificate.pem") {
        Ok(value) => value,
        Err(_) => panic!("Could not read the certificate.pem file - Please look at the readme.md"),
    };

    let key = match fs::read_to_string("certificate.key") {
        Ok(value) => value,
        Err(_) => panic!("Could not read the certificate.key file - Please look at the readme.md"),
    };

    let identity = Identity::from_pem(cert, key);
    let addr = "[::1]:5001".parse()?;

    let service = ExampleService::new();

    let shutdown = signal::ctrl_c();

    // Concurrently run the server and listen for the `shutdown` signal. The
    // server task runs until an error is encountered, so under normal
    // circumstances, this `select!` statement runs until the `shutdown` signal
    // is received.
    tokio::select! {
        res = Server::builder()
        .tls_config(ServerTlsConfig::new().identity(identity))?
        .add_service(ExampleServer::new(service))
        .serve(addr) => {
            if let Err(err) = res {
                log::error!("failed to start server - {:?}", err);
            }
        }
        _ = shutdown => {
            // The shutdown signal has been received.
            log::info!("shutting down");
        }
    }

    Ok(())
}
