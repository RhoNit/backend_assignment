use console::Style;
use dotenv::dotenv;
use tonic::transport::Server;

use landeed::{db, orders::v1::order_service_server::OrderServiceServer, services};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let mut connection = db::connection()?;
    db::run_migration(&mut connection);

    let addr = "0.0.0.0:50051".parse().unwrap();
    let order_service = services::orders::OrderServer::default();

    let blue = Style::new().blue();
    println!("\nRust gRPC Server ready at {}", blue.apply_to(addr));

    Server::builder()
        .add_service(OrderServiceServer::new(order_service))
        .serve(addr)
        .await?;

    Ok(())
}
