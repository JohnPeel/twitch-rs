use std::error::Error;

use twitch_rs::{ TwitchClient, api::helix::search::{CategorySearchRequest, CategoryGroup}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = TwitchClient::builder()
        .with_client_id(std::env::var("CLIENT_ID").expect("CLIENT_ID environment variable unset."))
        .with_client_secret(std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET environment variable unset."))
        .build().await?;
    
    let category = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "Just Chatting".to_owned());
    
    println!("Searching for \"{}\"...", &category);

    let mut category_list = vec![];
    let mut current_pagination = None;

    loop {
        let mut result = client.search.find_category(
            CategorySearchRequest::default()
                .with_query(&category)
                .with_after(current_pagination.clone())
        ).await?;

        if let Some(pagination) = result.pagination {
            current_pagination = pagination.cursor.clone();
        }

        category_list.append(&mut result.data);

        if current_pagination.is_none() {
            break;
        }
    }
    
    println!("{}", serde_json::to_string_pretty(&category_list)?);

    Ok(())
}
