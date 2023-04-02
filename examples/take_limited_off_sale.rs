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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = Client::new();
    client.set_roblosecurity(args.roblosecurity);

    let item_id = args.item_id;
    let uaid = args.uaid;

    let result = client.take_limited_off_sale(item_id, uaid).await;

    match result {
        Ok(()) => println!("Took item {} off sale.", item_id),
        Err(e) => println!("Failed to take item {} off sale. Reason: {}", item_id, e),
    }

    Ok(())
}
