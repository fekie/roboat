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

    let keyword = "linkmon".to_string();

    let users = client.user_search(keyword).await?;

    println!("Found {} users.", users.len());
    for user in users {
        println!("{}: {}", user.username, user.user_id);
    }

    Ok(())
}
