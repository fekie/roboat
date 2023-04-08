use roboat::ClientBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ClientBuilder::new().build();

    let collectible_item_id = client.collectible_item_id(13032232281).await?;

    println!("Ugc Limited Product ID: {}", collectible_item_id);

    Ok(())
}
