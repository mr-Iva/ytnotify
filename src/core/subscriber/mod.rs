use crate::core::types::channel::TwitchChannel;
use crate::core::types::telegram::BotUser;
use crate::db::dto::profile::NewUser;
use crate::db::dto::yt_noti_sub::NewYtNotiSub;
use crate::db::repo::profile_repo::ProfileRepo;
use crate::db::repo::yt_noti_sub_repo::YtNotiSubRepo;
use crate::yt_api;
use anyhow::Result;
use async_trait::async_trait;
use diesel::PgConnection;

#[async_trait]
pub trait Subscriber {
    // async fn unsubscribe(&self, channel_name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn subscribe(
        &self,
        user: BotUser,
        channel_name: String,
        pg: &mut PgConnection,
    ) -> Result<()>;
}

pub struct YtSubscriber {
    pub api_key: String,
}

impl YtSubscriber {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[async_trait]
impl Subscriber for YtSubscriber {
    async fn subscribe(
        &self,
        telegram_user: BotUser,
        channel_name: String,
        pg: &mut PgConnection,
    ) -> Result<()> {
        let api = yt_api::Api::new(&self.api_key.clone());
        let channel_id = api.get_channel_id(telegram_user.name.clone()).await;
        let new_user = NewUser {
            name: telegram_user.name,
            id: telegram_user.id.clone(),
        };
        let profile = ProfileRepo::create(&new_user, pg).unwrap();

        let new_yt_noti_sub = NewYtNotiSub {
            channel_name: channel_name.clone(),
            channel_id,
            profile_id: profile.id,
        };
        println!("new_yt_noti_sub: {:?}", new_yt_noti_sub);
        YtNotiSubRepo::create(&new_yt_noti_sub, pg).unwrap();
        Ok(())
    }
}

pub struct TwitchSubscriber {
    api_key: String,
}
#[async_trait]
impl Subscriber for TwitchChannel {
    async fn subscribe(
        &self,
        user: BotUser,
        channel_name: String,
        pg: &mut PgConnection,
    ) -> Result<()> {
        todo!()
    }
}
