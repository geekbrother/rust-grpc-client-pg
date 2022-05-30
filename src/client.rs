use glog::Flags;
use hello::hello_service_client::HelloServiceClient;
use hello::HelloRequest;
use log::info;

pub mod hello {
  tonic::include_proto!("hello");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  glog::new()
    .init(Flags {
      logtostderr: true, // don't write to log files
      ..Default::default()
    })
    .unwrap();
  // gRPC endpoint for grpcb.in hello
  let grpc_endpoint = "http://grpcb.in:9000/";
  info!(
    "Connecting to the grpcb.in endpoint at: {:?}",
    grpc_endpoint
  );
  let mut client = HelloServiceClient::connect(grpc_endpoint).await?;

  // Send gRPC request
  let request = tonic::Request::new(HelloRequest {
    greeting: "Max".into(),
  });

  let response = client.say_hello(request).await?;
  info!("Got gRPC response from gRPC server.");
  println!("gRPCb.in Response is: {:?}", response);
  Ok(())
}
