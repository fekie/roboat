use clap::Parser;
use serde::Serialize;

use roboat::ClientBuilder;

#[derive(clap::ValueEnum, Clone, Debug, Serialize)]
enum FriendRequestAction {
    Accept,
    Decline,
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    roblosecurity: String,

    #[arg(long, short)]
    requester_id: u64,

    #[arg(long, short)]
    action: FriendRequestAction,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = ClientBuilder::new()
        .roblosecurity(args.roblosecurity)
        .build();

    match args.action {
        FriendRequestAction::Accept => {
            match client.accept_friend_request(args.requester_id).await {
                Ok(_) => {
                    println!("Accepted!")
                }
                Err(err) => {
                    eprintln!("Error while accepting!");
                    eprintln!(" {}", err)
                }
            }
        }
        FriendRequestAction::Decline => {
            match client.decline_friend_request(args.requester_id).await {
                Ok(_) => {
                    println!("Declined!")
                }
                Err(err) => {
                    eprintln!("Error while declining!");
                    eprintln!(" {}", err)
                }
            }
        }
    }

    Ok(())
}
