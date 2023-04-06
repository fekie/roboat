use clap::Parser;
use roboat::Client;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short)]
    roblosecurity: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = Client::with_roblosecurity(args.roblosecurity);

    println!("Username: {}", client.username().await?);
    println!("Display Name: {}", client.display_name().await?);
    println!("User ID: {}", client.user_id().await?);

    Ok(())
}
