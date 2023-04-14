use clap::Parser;
use roboat::ClientBuilder;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short)]
    roblosecurity: String,
    #[arg(long, short)]
    item_id: u64,
    #[arg(long, short)]
    price: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = ClientBuilder::new()
        .roblosecurity(args.roblosecurity)
        .build();

    let item_id = args.item_id;
    let price = args.price;

    let collectible_item_id = client.collectible_item_id(item_id).await?;

    let collectible_product_id = client
        .collectible_product_id(collectible_item_id.clone())
        .await?;

    let collectible_creator_id = client
        .collectible_creator_id(collectible_item_id.clone())
        .await?;

    client
        .purchase_non_tradable_limited(
            collectible_item_id,
            collectible_product_id,
            collectible_creator_id,
            price,
        )
        .await?;

    println!("Purchased item {} for {} robux", item_id, price);

    Ok(())
}
