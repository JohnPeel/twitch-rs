use std::error::Error;

use serde::Serialize;
use reqwest::ClientBuilder;

use twitch_rs::{TwitchClient, api::helix::{pagination, clips::*, search::*}};

#[derive(Debug, Serialize)]
struct Clip {
    download_url: String,

    #[serde(flatten)]
    clip_info: GetClipsResponse
}

impl Clip {
    #[allow(dead_code)]
    async fn get_size(&self) -> Result<Option<u64>, Box<dyn Error>> {
        Ok(
            ClientBuilder::new().build()?
                .head(&self.download_url)
                .send()
                .await?
                .headers()
                .get("content-length")
                .map(|x| x.to_str())
                .transpose()?
                .map(|x| x.parse::<u64>())
                .transpose()?
        )
    }
}

impl From<GetClipsResponse> for Clip {
    fn from(x: GetClipsResponse) -> Self {
        Self {
            download_url: format!("{}.mp4", &x.thumbnail_url.rsplitn(3, '-').nth(2).unwrap()),
            clip_info: x
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client: TwitchClient = TwitchClient::builder()
        .with_client_id(std::env::var("CLIENT_ID").expect("CLIENT_ID environment variable unset."))
        .with_client_secret(std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET environment variable unset."))
        .build().await?;
    
    let requested_channel: String = std::env::args()
        .nth(1)
        .expect("A channel name was not supplied.");
    
    let channel_list: Vec<ChannelResponse> = client.search.find_channel(
        ChannelSearchRequest::default()
            .with_query(&requested_channel)
    ).await?.data;

    let channel: &ChannelResponse = channel_list.get(0).expect("Cannot find channel.");

    let clips_list: Vec<Clip> = pagination::get_all(|page| {
        client.clips.get_clips(
            GetClipsRequest::default()
                .with_broadcaster_id(&channel.id)
                .with_after(page)
        )
    }, None).await?.into_iter().map(Clip::from).collect();

    println!("{:?}", &clips_list);

    Ok(())
}
