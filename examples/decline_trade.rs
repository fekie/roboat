use clap::Parser;
use roboat::ClientBuilder;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short)]
    roblosecurity: String,
    #[arg(long, short)]
    trade_id: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = ClientBuilder::new()
        .roblosecurity(args.roblosecurity)
        .build();

    let trade_id = args.trade_id;

    client.decline_trade(trade_id).await?;

    println!("Declined trade {}", trade_id);

    Ok(())
}
