use clap::Parser;
use roboat::ClientBuilder;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short)]
    roblosecurity: String,
    #[arg(long, short)]
    group_id: u64,
    #[arg(long, short)]
    user_id: u64,
    #[arg(long, short)]
    new_role_id: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = ClientBuilder::new()
        .roblosecurity(args.roblosecurity)
        .build();

    let group_id = args.group_id;
    let user_id = args.user_id;
    let new_role_id = args.new_role_id;

    client
        .set_group_member_role(user_id, group_id, new_role_id)
        .await?;

    println!(
        "Set user {}'s role to role id {} in group {}.",
        user_id, new_role_id, group_id
    );

    Ok(())
}
