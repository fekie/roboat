use clap::Parser;
use roboat::ClientBuilder;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short)]
    roblosecurity: String,

    #[arg(long, short)]
    target_id: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = ClientBuilder::new()
        .roblosecurity(args.roblosecurity)
        .build();

    match client.send_friend_request(args.target_id).await {
        Ok(_) => {
            println!("Sent!")
        }
        Err(err) => {
            eprintln!("Error while sending!");
            eprintln!(" {}", err)
        }
    }

    Ok(())
}
