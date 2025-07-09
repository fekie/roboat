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
    let client = ClientBuilder::new().build();

    let friends_list = client.friends_list(args.user_id).await?;
    for friend in friends_list {
        println!("{}: {} ", friend.display_name, friend.id);
    }

    Ok(())
}
