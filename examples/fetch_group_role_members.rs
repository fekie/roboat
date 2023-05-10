use clap::Parser;
use roboat::{ClientBuilder, Limit};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short)]
    group_id: u64,
    #[arg(long, short)]
    role_id: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = ClientBuilder::new().build();

    let mut next_cursor = None;

    // Print up to 100 pages of members in a role.
    for i in 0..100 {
        let (members, cursor) = client
            .group_role_members(args.group_id, args.role_id, Limit::Hundred, next_cursor)
            .await?;

        next_cursor = cursor;

        println!("Page {} of Members:", i);

        for member in members {
            println!(
                "User ID: {} / Username: {} / Display Name: {}",
                member.user_id, member.username, member.display_name
            );
        }

        if next_cursor.is_none() {
            break;
        }
    }

    Ok(())
}
