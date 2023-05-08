use roboat::catalog::avatar_catalog::ItemArgs;
use roboat::catalog::avatar_catalog::ItemType;
use roboat::ClientBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // bundle
    let item_1 = ItemArgs {
        item_type: ItemType::Bundle,
        id: 39,
    };

    // limited item
    let item_2 = ItemArgs {
        item_type: ItemType::Asset,
        id: 1365767,
    };

    // ugc
    let item_3 = ItemArgs {
        item_type: ItemType::Asset,
        id: 12415326352,
    };

    // off sale
    let item_4 = ItemArgs {
        item_type: ItemType::Asset,
        id: 11386336162,
    };

    // no resellers
    let item_5 = ItemArgs {
        item_type: ItemType::Asset,
        id: 148791559,
    };

    // limited u
    let item_6 = ItemArgs {
        item_type: ItemType::Asset,
        id: 21070789,
    };

    let items = vec![item_1, item_2, item_3, item_4, item_5, item_6];

    let client = ClientBuilder::new().build();
    let details = client.item_details(items).await?;

    println!(
        "Bundle Name: {} / Bundle Price: {}",
        details[0].name,
        details[0]
            .price
            .map(|x| x.to_string())
            .unwrap_or_else(|| "*No Resellers*".to_owned())
    );

    println!(
        "Item Name: {} / Item Price: {}",
        details[1].name,
        details[1]
            .price
            .map(|x| x.to_string())
            .unwrap_or_else(|| "*No Resellers*".to_owned())
    );

    println!(
        "Item Name: {} / Item Price: {}",
        details[2].name,
        details[2]
            .price
            .map(|x| x.to_string())
            .unwrap_or_else(|| "*No Resellers*".to_owned())
    );

    println!(
        "Item Name: {} / Item Price: {}",
        details[3].name,
        details[3]
            .price
            .map(|x| x.to_string())
            .unwrap_or_else(|| "*No Resellers*".to_owned())
    );

    println!(
        "Item Name: {} / Item Price: {}",
        details[4].name,
        details[4]
            .price
            .map(|x| x.to_string())
            .unwrap_or_else(|| "*No Resellers*".to_owned())
    );

    println!(
        "Item Name: {} / Item Price: {}",
        details[5].name,
        details[5]
            .price
            .map(|x| x.to_string())
            .unwrap_or_else(|| "*No Resellers*".to_owned())
    );

    Ok(())
}
