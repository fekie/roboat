use roboat::catalog::{Item, ItemType};
use roboat::ClientBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // bundle
    let item_1 = Item {
        item_type: ItemType::Bundle,
        id: 39,
    };

    // limited item
    let item_2 = Item {
        item_type: ItemType::Asset,
        id: 1365767,
    };

    // ugc
    let item_3 = Item {
        item_type: ItemType::Asset,
        id: 12415326352,
    };

    // off sale
    let item_4 = Item {
        item_type: ItemType::Asset,
        id: 11386336162,
    };

    // no resellers
    let item_5 = Item {
        item_type: ItemType::Asset,
        id: 148791559,
    };

    // limited u
    let item_6 = Item {
        item_type: ItemType::Asset,
        id: 21070789,
    };

    // ugc limited
    let item_7 = Item {
        item_type: ItemType::Asset,
        id: 13464465797,
    };

    // off sale ugc (may be out of date in the future)
    let item_8 = Item {
        item_type: ItemType::Asset,
        id: 13450447846,
    };

    // unsold ugc
    let item_9 = Item {
        item_type: ItemType::Asset,
        id: 13600952997,
    };

    // unsold ugc with resellers
    let item_10 = Item {
        item_type: ItemType::Asset,
        id: 13420878571,
    };

    // sold out ugc with resellers
    let item_11 = Item {
        item_type: ItemType::Asset,
        id: 12073669026,
    };

    let items = vec![
        item_1, item_2, item_3, item_4, item_5, item_6, item_7, item_8, item_9, item_10, item_11,
    ];

    let client = ClientBuilder::new().build();
    let all_details = client.item_details(items).await?;

    for details in all_details {
        println!("Item Id: {}", details.id);
        println!("{:?}", details);
        println!("\n");
    }

    Ok(())
}
