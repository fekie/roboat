use roboat::thumbnails::AssetThumbnailSize;
use roboat::ClientBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let avatar_id_1 = 2207291;
    let avatar_id_2 = 15189;

    let size = AssetThumbnailSize::S420x420;

    let client = ClientBuilder::new().build();
    let urls = client
        .avatar_thumbnail_url_bulk(vec![avatar_id_1, avatar_id_2], size)
        .await?;

    println!("Avatar {} thumbnail url: {}", avatar_id_1, urls[0]);
    println!("Avatar {} thumbnail url: {}", avatar_id_2, urls[1]);

    Ok(())
}
