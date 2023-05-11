use clap::Parser;
use roboat::trades::TradeType;
use roboat::ClientBuilder;
use roboat::Limit;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short)]
    roblosecurity: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = ClientBuilder::new()
        .roblosecurity(args.roblosecurity)
        .build();

    let trade_type = TradeType::Inbound;
    let limit = Limit::Ten;
    let cursor = None;

    let (trades, _next_cursor) = client.trades(trade_type, limit, cursor).await?;

    let trade_count = trades.len();

    match trade_count {
        0 => println!("No inbound trades found."),
        _ => {
            for (i, trade) in trades.iter().enumerate() {
                println!(
                    "Inbound Trade #{} Partner: {}",
                    i + 1,
                    trade.partner.username
                );
            }
        }
    }

    Ok(())
}
