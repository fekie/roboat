use clap::Parser;
use roboat::ClientBuilder;

use std::fs::File;
use std::io::Write;
use std::path::Path;
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
    let data_blob = client.fetch_asset_data(asset_id).await?;

    println!("Got asset data: {:?}", data_blob);

    // Save to file
    let filename = format!("asset_{}.bin", asset_id);
    let path = Path::new(&filename);
    let mut file = File::create(&path)?;
    file.write_all(&data_blob)?;

    println!("Saved asset to {}", filename);

    Ok(())
}
