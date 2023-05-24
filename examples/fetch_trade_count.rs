use clap::Parser;
use roboat::ClientBuilder;
use roboat::RoboatError;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short)]
    roblosecurity: String,
}

#[tokio::main]
async fn main() -> Result<(), RoboatError> {
    let args = Args::parse();
    let client = ClientBuilder::new()
        .roblosecurity(args.roblosecurity)
        .build();

    let trade_count = client.trade_count().await?;

    println!("Total trades: {}", trade_count);

    Ok(())
}
