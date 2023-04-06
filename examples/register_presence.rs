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

    let result = client.register_presence().await;

    match result {
        Ok(()) => println!("Registered presence!"),
        Err(e) => println!("Failed to register presence. Reason: {}", e),
    }

    Ok(())
}
