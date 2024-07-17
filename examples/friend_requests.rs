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

    let pending_friend_requests = client.pending_friend_requests().await?;

    println!("Found {} pending requests.", pending_friend_requests);

    // iterate through all friend requests

    let mut current_cursor = None;

    loop {
        let (friend_requests, next_cursor) = client.friend_requests(current_cursor).await?;

        for user in friend_requests {
            println!(
                " - {} from {}: {}",
                user.username, user.origin_source_type, user.user_id
            );
        }

        if let Some(cursor) = next_cursor {
            current_cursor = Some(cursor)
        } else {
            break;
        }
    }

    Ok(())
}
