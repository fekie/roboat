use clap::Parser;
use roboat::{Client, Limit};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short)]
    roblosecurity: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let client = Client::new();
    client.set_roblosecurity(args.roblosecurity);

    let limit = Limit::Hundred;
    let cursor = None;

    let (user_sales, _) = client.user_sales(limit, cursor).await?;

    println!(
        "Robux gained from last {} sales: {}",
        user_sales.len(),
        user_sales
            .iter()
            .map(|sale| sale.robux_received)
            .sum::<u64>()
    );

    Ok(())
}
