use std::error::Error;

use twitch_rs::{ TwitchClient, api::helix::search::{ChannelSearchRequest, ChannelGroup}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = TwitchClient::builder()
        .with_client_id(std::env::var("CLIENT_ID").expect("CLIENT_ID environment variable unset."))
        .with_client_secret(std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET environment variable unset."))
        .build().await?;
    
    let channel = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "dgby714".to_owned());
    
    println!("Searching for \"{}\"...", &channel);

    let mut channel_list = vec![];
    let mut current_pagination = None;

    loop {
        let mut result = client.search.find_channel(
            ChannelSearchRequest::default()
                .with_query(&channel)
                .with_after(current_pagination.clone())
        ).await?;

        if let Some(pagination) = result.pagination {
            current_pagination = pagination.cursor.clone();
        }

        channel_list.append(&mut result.data);

        if current_pagination.is_none() || channel_list.len() > 50 {
            break;
        }
    }

    channel_list.into_iter()
        .map(|x| x.display_name)
        .for_each(|x| println!("Found \"{}\".", x));

    Ok(())
}
