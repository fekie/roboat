use roboat::ClientBuilder;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short)]
    roblosecurity: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let collectible_item_id = "61f2e366-9fe6-4562-8ce3-47334083372a".to_owned();

    let client = ClientBuilder::new()
        .roblosecurity(args.roblosecurity)
        .build();

    let collectible_product_id = client.collectible_product_id(collectible_item_id).await?;

    println!("Collectible Product ID: {}", collectible_product_id);

    Ok(())
}
