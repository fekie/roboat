use roboat::thumbnails::{AssetThumbnailSize, ThumbnailType};
use roboat::ClientBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let avatar_id_1 = 2207291;
    let avatar_id_2 = 15189;
    let avatar_id_3 = 5866753;

    let size = AssetThumbnailSize::S420x420;
    let thumbnail_type = ThumbnailType::AvatarHeadshot;

    let client = ClientBuilder::new().build();

    // Either of these methods work, both are here just to show the two different ways to do it.
    let urls = client
        .thumbnail_url_bulk(vec![avatar_id_1, avatar_id_2], size, thumbnail_type)
        .await?;

    let url = client
        .thumbnail_url(avatar_id_3, size, thumbnail_type)
        .await?;

    println!("Avatar headshot {} thumbnail url: {}", avatar_id_1, urls[0]);
    println!("Avatar headshot {} thumbnail url: {}", avatar_id_2, urls[1]);
    println!("Avatar headshot {} thumbnail url: {}", avatar_id_3, url);

    Ok(())
}
