use roboat::catalog::{AvatarSearchQueryBuilder, CatalogQueryLimit, Category, SalesTypeFilter};
use roboat::{ClientBuilder, RoboatError};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let query = AvatarSearchQueryBuilder::new()
        .keyword("cute".to_owned())
        .min_price(300)
        .sales_type_filter(SalesTypeFilter::Collectibles)
        .limit(CatalogQueryLimit::TwentyEight)
        .category(Category::Accessories)
        .build();

    let client = ClientBuilder::new().build();

    let mut next_cursor = None;

    loop {
        let (items, cursor) = match client
            .avatar_catalog_search(&query, next_cursor.clone())
            .await
        {
            Ok(x) => x,
            Err(e) => match e {
                RoboatError::TooManyRequests => {
                    println!("Too many requests, waiting 60 seconds...");
                    std::thread::sleep(std::time::Duration::from_secs(60));
                    continue;
                }
                _ => return Err(e.into()),
            },
        };

        println!("Items on this page: {}", items.len());

        if cursor.is_none() || items.is_empty() {
            break;
        }

        next_cursor = cursor;
    }

    Ok(())
}
