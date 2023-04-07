use clap::Parser;
use roboat::ClientBuilder;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short)]
    roblosecurity: String,
    #[arg(long, short)]
    item_id: u64,
    #[arg(long, short)]
    seller_id: u64,
    #[arg(long, short)]
    uaid: u64,
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
    let seller_id = args.seller_id;
    let uaid = args.uaid;
    let price = args.price;

    let item_args = roboat::catalog::avatar_catalog::ItemArgs {
        item_type: roboat::catalog::avatar_catalog::ItemType::Asset,
        id: item_id,
    };

    let product_id = client
        .item_details(vec![item_args])
        .await?
        .pop()
        .unwrap()
        .product_id
        .expect("Item cannot be a \"new\" limited. Run purchase_ugc_limited instead.");

    let result = client
        .purchase_limited(product_id, seller_id, uaid, price)
        .await;

    match result {
        Ok(()) => println!("Purchased item for {} robux.", price),
        Err(e) => println!("Failed to purchase item for {} robux. Reason: {}", price, e),
    }

    Ok(())
}
