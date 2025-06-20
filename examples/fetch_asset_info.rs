use clap::Parser;
use roboat::ClientBuilder;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short)]
    roblosecurity: String,
    #[arg(long, short)]
    asset_id: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let client = ClientBuilder::new()
        .roblosecurity(args.roblosecurity)
        .build();

    let asset_id = args.asset_id;
    let asset_info = client.get_asset_info(asset_id).await?;

    println!("Got asset data: {:?}", asset_info.asset_type);

    Ok(())
}
