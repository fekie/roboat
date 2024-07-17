use clap::Parser;

use roboat::presence::PresenceType;
use roboat::ClientBuilder;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short)]
    user_id: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = ClientBuilder::new().build();

    let friends = client.friends_list(args.user_id).await?;

    println!("Found {} friends.", friends.len());
    for friend in friends {
        print!("{}: {} ", friend.username, friend.user_id);
        if friend.presence_type != PresenceType::Offline {
            println!("({:?})", friend.presence_type)
        } else {
            println!();
        }
    }

    Ok(())
}
