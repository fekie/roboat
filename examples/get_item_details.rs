use roboat::catalog::avatar_catalog::ItemParameters;
use roboat::catalog::avatar_catalog::ItemType;
use roboat::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // bundle
    let item_1 = ItemParameters {
        item_type: ItemType::Bundle,
        id: 39,
    };

    // limited item
    let item_2 = ItemParameters {
        item_type: ItemType::Asset,
        id: 1365767,
    };

    // ugc
    let item_3 = ItemParameters {
        item_type: ItemType::Asset,
        id: 12415326352,
    };

    // off sale
    let item_4 = ItemParameters {
        item_type: ItemType::Asset,
        id: 11386336162,
    };

    // no resellers
    let item_5 = ItemParameters {
        item_type: ItemType::Asset,
        id: 148791559,
    };

    let client = Client::new();
    let details = client
        .item_details(vec![item_1, item_2, item_3, item_4, item_5])
        .await?;

    println!(
        "Bundle Name: {} / Bundle Price: {}",
        details[0].name,
        details[0].price.unwrap()
    );

    println!(
        "Item Name: {} / Limited Item Price: {}",
        details[1].name,
        details[1].price.unwrap()
    );

    println!(
        "Item Name: {} / UGC Item Price: {}",
        details[2].name,
        details[2].price.unwrap()
    );

    println!(
        "Item Name: {} / Off Sale Item Price: {}",
        details[3].name,
        details[3].price.unwrap()
    );

    println!(
        "Item Name: {} / Item Price: {}",
        details[4].name,
        details[4]
            .price
            .map(|x| x.to_string())
            .unwrap_or_else(|| "*No Resellers*".to_owned())
    );

    Ok(())
}
