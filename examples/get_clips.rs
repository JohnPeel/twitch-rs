use std::error::Error;

use twitch_rs::{ TwitchClient, api::helix::clips::* };

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = TwitchClient::builder()
        .with_client_id(std::env::var("CLIENT_ID").expect("CLIENT_ID environment variable unset."))
        .with_client_secret(std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET environment variable unset."))
        .build().await?;
    
    let ids: Vec<String> = std::env::args()
        .skip(1)
        .collect();
    
    println!("{}", serde_json::to_string_pretty(&client.clips.get_clips(
        GetClipsRequest::default()
            .with_ids(ids)
    ).await?)?);

    Ok(())
}
