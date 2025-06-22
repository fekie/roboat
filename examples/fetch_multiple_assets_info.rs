use clap::Parser;
use roboat::assetdelivery::request_types::AssetBatchPayload;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short)]
    roblosecurity: String,
}

use roboat::ClientBuilder;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = ClientBuilder::new()
        .roblosecurity(args.roblosecurity)
        .build();
    let payloads = vec![
        AssetBatchPayload {
            asset_id: Some("105277031944789".to_string()),
            request_id: Some("0".to_string()),
            ..Default::default()
        },
        AssetBatchPayload {
            asset_id: Some("1031944789".to_string()),
            request_id: Some("0".to_string()),
            ..Default::default()
        },
    ];
    let responses = client.post_asset_metadata_batch(payloads).await?;
    for response in responses {
        println!("Response for request {:?}", response);
    }
    Ok(())
}
