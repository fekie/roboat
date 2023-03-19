#[tokio::main]
async fn main() {
    use roboat::Client;

    let client = Client::new();
    let details = client.item_details().await;
}
