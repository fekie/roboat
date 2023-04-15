use roboat::ClientBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ClientBuilder::new().build();

    // There is not really another way to debug this other than printing the xcsrf inside the source code for this method.
    client.force_refresh_xcsrf().await?;
    client.force_refresh_xcsrf().await?;
    client.force_refresh_xcsrf().await?;

    Ok(())
}
