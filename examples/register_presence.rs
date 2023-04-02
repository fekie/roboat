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

    let result = client.register_presence().await;

    match result {
        Ok(()) => println!("Registered presence!"),
        Err(e) => println!("Failed to register presence. Reason: {}", e),
    }

    Ok(())
}
