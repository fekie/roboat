use clap::Parser;
use roboat::ClientBuilder;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short)]
    group_id: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = ClientBuilder::new().build();

    let roles = client.group_roles(args.group_id).await?;

    // Print all roles in order by rank.
    for role in roles {
        println!(
            "Role: {} / ID: {} / Rank: {}",
            role.name, role.id, role.rank
        );
    }

    Ok(())
}
