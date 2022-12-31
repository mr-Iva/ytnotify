use crate::telegram_bot::start_bot;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Bot {
    async fn start() -> Result<()>;
}

pub struct TelegramBot {}

#[async_trait]
impl Bot for TelegramBot {
    async fn start() -> Result<()> {
        start_bot();
        Ok(())
    }
}
