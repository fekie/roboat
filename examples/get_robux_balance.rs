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

    let client = Client::new();
    client.set_roblosecurity(args.roblosecurity);

    let user = client.username().await?;
    let robux = client.robux_balance().await?;

    println!("Robux for {}: {}", user, robux);

    Ok(())
}
