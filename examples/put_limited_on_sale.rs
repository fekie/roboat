use clap::Parser;
use roboat::Client;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short)]
    roblosecurity: String,
    #[arg(long, short)]
    item_id: u64,
    #[arg(long, short)]
    uaid: u64,
    #[arg(long, short)]
    price: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = Client::with_roblosecurity(args.roblosecurity);

    let item_id = args.item_id;
    let uaid = args.uaid;
    let price = args.price;

    let result = client.put_limited_on_sale(item_id, uaid, price).await;

    match result {
        Ok(()) => println!("Placed item {} on sale for {} robux.", item_id, price),
        Err(e) => println!(
            "Failed to put item {} on sale for {} robux. Reason: {}",
            item_id, price, e
        ),
    }

    Ok(())
}
