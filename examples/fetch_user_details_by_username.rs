use roboat::ClientBuilder;

const USERNAME: &str = "Builderman";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ClientBuilder::new().build();

    let users = vec![USERNAME.to_owned()];
    let all_username_user_details = client.username_user_details(users, Some(true)).await?;
    let username_user_details = all_username_user_details.first().ok_or("User not found")?;

    println!("Username: {}", username_user_details.username);
    println!("Display Name: {}", username_user_details.display_name);
    println!("ID: {}", username_user_details.id);

    Ok(())
}
