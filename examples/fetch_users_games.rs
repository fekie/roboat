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

    let games_response = client.user_games(args.user_id).await?;
    for game in games_response.data {
        println!("Game: {} (ID: {})", game.name, game.id);
        println!("  Root  ID: {}", game.root_place.id);
        println!("  Visits: {}", game.place_visits);
        println!("  Last Updated: {}", game.updated);
    }
    // Check for more pages
    if let Some(next_cursor) = games_response.next_page_cursor {
        println!("More results available with cursor: {}", next_cursor);
    }

    Ok(())
}
