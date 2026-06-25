use scanner_grpc::bsap::scanner_service_server::ScannerServiceServer;
use scanner_grpc::BsapScannerServer;
use tonic::transport::Server;
use tonic_health::server::health_reporter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let (mut health_reporter, health_service) = health_reporter();
    health_reporter.set_serving::<ScannerServiceServer<BsapScannerServer>>().await;

    let addr = "[::1]:50051".parse()?;
    println!("🚀 BSAP gRPC Scanner Server listening on {}", addr);

    Server::builder()
        .add_service(health_service)
        .add_service(ScannerServiceServer::new(BsapScannerServer::default()))
        .serve(addr)
        .await?;

    Ok(())
}
