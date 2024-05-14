use clap::Parser;
use roboat::ClientBuilder;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short)]
    roblosecurity: String,
    #[arg(long, short)]
    group_id: u64,
    #[arg(long, short)]
    name: String,
    #[arg(long, short)]
    description: String,
    #[arg(long, short)]
    image_path: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = ClientBuilder::new()
        .roblosecurity(args.roblosecurity)
        .build();

    let classic_clothing_type = roboat::bedev2::ClassicClothingType::Shirt;

    let result = client
        .upload_classic_clothing_to_group(
            args.group_id,
            args.name,
            args.description,
            args.image_path,
            classic_clothing_type,
        )
        .await;

    match result {
        Ok(()) => println!("Uploaded shirt successfully."),
        Err(e) => println!("Failed to upload shirt. Reason: {}", e),
    }

    Ok(())
}
