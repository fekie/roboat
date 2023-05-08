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

    let user_details = client.user_details(args.user_id).await?;

    println!("Username: {}", user_details.username);
    println!("Display Name: {}", user_details.display_name);
    println!(
        "Year Created: {}",
        user_details.created_at.chars().take(4).collect::<String>()
    );

    Ok(())
}
