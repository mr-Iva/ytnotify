use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SearchItems {
    #[serde(rename = "items")]
    items: Vec<Item>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Item {
    kind: String,
    etag: String,
    pub(crate) id: ItemKind,
    snippet: Option<Snippet>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Snippet {
    title: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct ChannelId {
    #[serde(rename = "channelId")]
    channel_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct VideoId {
    #[serde(rename = "videoId")]
    video_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
enum ItemKind {
    Channel(ChannelId),
    Video(VideoId),
}

#[derive(Debug)]
pub struct Api {
    pub api_key: String,
}

impl Api {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
        }
    }
    fn from_json_str(json_str: &str, channel_name: &str, api_key: String) -> SearchItems {
        let searched_item = serde_json::from_str::<SearchItems>(json_str).unwrap();
        searched_item
    }
    //api key AIzaSyDSCK4V0xq19-RJlok3LWwMUulVrlfIXAk
    pub async fn get_channel_id(&self, channel_name: String) -> String {
        let api_key = self.api_key.clone();
        let resp = reqwest::get(format!(
            "https://youtube.googleapis.com/youtube/v3/search?part=id&q={channel_name}&key={api_key}&type=channel"
        )).await.unwrap();
        println!("{:?}", resp);
        let search_items = Self::from_json_str(
            resp.text().await.unwrap().as_str(),
            channel_name.as_str(),
            api_key,
        );
        // let channel_id = string;
        // resp.text().await.unwrap()
        match &search_items.items[0].id {
            ItemKind::Channel(channel_id) => channel_id.channel_id.clone(),
            ItemKind::Video(video_id) => {
                panic!("video id: {}", video_id.video_id);
            }
        }
    }
    pub async fn get_videos(&self, channel_id: &str, max_results: u32) -> String {
        let api_key = self.api_key.clone();
        let resp = reqwest::get(format!(
            "https://www.googleapis.com/youtube/v3/search?key={api_key}&channelId={channel_id}&part=snippet,id&order=date&maxResults={max_results}&type=video"
        )).await.unwrap();
        let json = resp.text().await.unwrap();
        let items = serde_json::from_str::<SearchItems>(&*json).unwrap().items;
        serde_json::to_string(&items).unwrap()
    }
}
