use clap::Parser;
use roboat::ClientBuilder;

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

    println!("Username: {}", client.username().await?);
    println!("Display Name: {}", client.display_name().await?);
    println!("User ID: {}", client.user_id().await?);

    Ok(())
}
