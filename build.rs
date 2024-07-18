fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .compile(&["protos/orders/v1/orders.proto"], &["protos"])?;
    Ok(())
}
