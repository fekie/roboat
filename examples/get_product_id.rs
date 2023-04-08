use roboat::ClientBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ClientBuilder::new().build();

    let product_id = client.product_id(1365767).await?;

    println!("Ugc Limited Product ID: {}", product_id);

    Ok(())
}
