use clap::Parser;
use roboat::ClientBuilder;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short)]
    user_id: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = ClientBuilder::new()
        .build();

    let friends = client.friends_list(args.user_id).await?;

    println!("Found {} friends.", friends.len());
    for friend in friends {
        println!("{}: {}", friend.username, friend.user_id);
    }

    Ok(())
}
