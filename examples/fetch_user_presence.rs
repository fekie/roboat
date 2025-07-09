use roboat::ClientBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ClientBuilder::new().build();

    let user_list = vec![1, 53427532];
    let users_presences = client.fetch_users_presence(user_list).await?;

    println!("{:?}", users_presences);

    Ok(())
}
