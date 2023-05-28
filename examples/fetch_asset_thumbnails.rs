use roboat::thumbnails::{ThumbnailSize, ThumbnailType};
use roboat::ClientBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let asset_id_1 = 20418400;
    let asset_id_2 = 12660007639;
    let asset_id_3 = 11377306;

    let size = ThumbnailSize::S420x420;
    let thumbnail_type = ThumbnailType::Asset;

    let client = ClientBuilder::new().build();

    // Either of these methods work, both are here just to show the two different ways to do it.
    let urls = client
        .thumbnail_url_bulk(vec![asset_id_1, asset_id_2], size, thumbnail_type)
        .await?;

    let url = client
        .thumbnail_url(asset_id_3, size, thumbnail_type)
        .await?;

    println!("Asset {} thumbnail url: {}", asset_id_1, urls[0]);
    println!("Asset {} thumbnail url: {}", asset_id_2, urls[1]);
    println!("Asset {} thumbnail url: {}", asset_id_3, url);

    Ok(())
}
