use clap::Parser;
use roboat::Client;
use roboat::Limit;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short)]
    roblosecurity: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let client = Client::with_roblosecurity(args.roblosecurity);

    let item_id = 1365767;
    let limit = Limit::Ten;
    let cursor = None;

    let (resellers, _) = client.resellers(item_id, limit, cursor).await?;

    println!("Lowest Price for Item {}: {}", item_id, resellers[0].price);

    Ok(())
}
