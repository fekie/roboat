use clap::Parser;
use roboat::ClientBuilder;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short)]
    roblosecurity: String,
    #[arg(long)]
    partner_id: u64,
    #[arg(long, num_args = 1.., value_delimiter = ',', required = true)]
    your_uaids: Vec<u64>,
    #[arg(long)]
    your_robux: u64,
    #[arg(long, num_args = 1.., value_delimiter = ',', required = true)]
    partner_uaids: Vec<u64>,
    #[arg(long)]
    partner_robux: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = ClientBuilder::new()
        .roblosecurity(args.roblosecurity.clone())
        .build();

    let partner_id = args.partner_id;
    let your_uaids = args.your_uaids;
    let your_robux = args.your_robux;
    let partner_uaids = args.partner_uaids;
    let partner_robux = args.partner_robux;

    let trade_id = client
        .send_trade(
            partner_id,
            your_uaids,
            your_robux,
            partner_uaids,
            partner_robux,
        )
        .await?;

    println!("Sent Trade! Trade ID: {}", trade_id);

    Ok(())
}
