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

    let collectible_item_id_1 = "a4b5cb79-5218-4ca1-93fa-1e3436f595ef".to_owned();
    let collectible_item_id_2 = "61f2e366-9fe6-4562-8ce3-47334083372a".to_owned();

    let items = vec![collectible_item_id_1, collectible_item_id_2];

    let client = ClientBuilder::new()
        .roblosecurity(args.roblosecurity)
        .build();

    let details = client.non_tradable_limited_details(items).await?;

    dbg!(details);

    Ok(())
}
