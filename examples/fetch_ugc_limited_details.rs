use roboat::catalog::{Item, ItemType};
use roboat::ClientBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Ugc limited
    let item_1 = Item {
        item_type: ItemType::Asset,
        id: 13032232281,
    };

    let items = vec![item_1];

    let client = ClientBuilder::new().build();
    let details = client.item_details(items).await?;

    println!(
        "Ugc Limited Name: {} / Ugc Limited Collectible Id: {}",
        details[0].name,
        details[0]
            .collectible_item_id
            .as_ref()
            .map(|x| x.to_string())
            .unwrap_or_else(|| "Collectible id not found.".to_owned())
    );

    Ok(())
}
