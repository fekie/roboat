use clap::Parser;
use roboat::{ide::ide_types::NewAnimation, ClientBuilder};

/// This example is made for programs that would restore old roblox games
/// Whenever you download a .rblx file and it has animations none of them will work.
/// This is because the animations isn't owned by you.
/// Reuploading them will succesfully restore the animations under a new ID.
///
#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short)]
    roblosecurity: String,
    #[arg(long, short)]
    asset_id: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let client = ClientBuilder::new()
        .roblosecurity(args.roblosecurity)
        .build();

    let asset_id = args.asset_id;
    let existing_animation = client.fetch_asset_data(asset_id).await?;

    let animation = NewAnimation {
        group_id: None,
        name: "roboatTest".to_string(),
        description: "This is a roboat example".to_string(),
        animation_data: existing_animation,
    };

    client.upload_new_animation(animation).await?;

    println!("Uploaded Animation!");
    Ok(())
}
