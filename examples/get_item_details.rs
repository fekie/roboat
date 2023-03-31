use roboat::catalog::avatar_catalog::ItemParameters;
use roboat::catalog::avatar_catalog::ItemType;
use roboat::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let item_1 = ItemParameters {
        item_type: ItemType::Bundle,
        id: 39,
    };

    let client = Client::new();
    let details = client.item_details(vec![item_1]).await?;
    println!(
        "Bundle Name: {} / Bundle Price: {}",
        details[0].name, details[0].price
    );

    Ok(())
}
