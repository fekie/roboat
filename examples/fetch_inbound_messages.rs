use clap::Parser;
use roboat::private_messages::MessageTabType::Inbox;
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

    let inbox_type = Inbox;

    let (messages, messages_metadata) = client.messages(0, inbox_type).await?;

    println!("First Message Subject: {}", messages[0].subject);
    println!("Total Messages: {}", messages_metadata.total_message_count);
    println!("Total Pages: {}", messages_metadata.total_pages);

    Ok(())
}
