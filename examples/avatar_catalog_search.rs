use roboat::catalog::avatar_catalog::search_query::{AvatarSearchQuery, AvatarSearchQueryBuilder};
use roboat::catalog::avatar_catalog::ItemArgs;
use roboat::ClientBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let query = AvatarSearchQueryBuilder::new()
        .keyword("cute".to_owned())
        .build();

    let client = ClientBuilder::new().build();

    let foo = client.avatar_catalog_search(query).await?;

    Ok(())
}
