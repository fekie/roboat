use roboat::thumbnails::AssetThumbnailSize;
use roboat::ClientBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let asset_id_1 = 20418400;
    let asset_id_2 = 12660007639;

    let size = AssetThumbnailSize::S420x420;

    let client = ClientBuilder::new().build();
    let urls = client
        .asset_thumbnail_url_bulk(vec![asset_id_1, asset_id_2], size)
        .await?;

    println!("Asset {} thumbnail url: {}", asset_id_1, urls[0]);
    println!("Asset {} thumbnail url: {}", asset_id_2, urls[1]);

    Ok(())
}
