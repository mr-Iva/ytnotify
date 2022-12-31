use anyhow::Result;
use diesel::PgConnection;
use regex::Regex;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use teloxide::{prelude::*, utils::command::BotCommands};

use crate::core::subscriber::{Subscriber, YtSubscriber};
use crate::core::types::telegram::{BotUser, State};
use crate::telegram_bot::Db;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Available commands:")]
pub enum Command {
    #[command(description = "help")]
    Help,
    #[command(description = "Add notification for channel. Using /addnotify <channel_name>")]
    AddNotify(String),
}

pub async fn answer(bot: Bot, msg: Message, cmd: Command, pg: Db) -> Result<()> {
    let api_token = "AIzaSyDSCK4V0xq19-RJlok3LWwMUulVrlfIXAk";
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::AddNotify(channel_url) => {
            let re_yt = Regex::new(r"https://www.youtube.com/").unwrap();
            let re_tw = Regex::new(r"https://www.twitch.tv/").unwrap();

            if re_yt.is_match(&channel_url) {
                let api = YtSubscriber::new(api_token.to_string());
                let api_result = api
                    .subscribe(
                        BotUser {
                            id: msg.chat.id.to_string(),
                            name: msg.chat.username().unwrap().to_string(),
                        },
                        channel_url.clone(),
                        pg.conn.lock().unwrap().deref_mut(),
                    )
                    .await;
                match api_result {
                    Ok(_) => {
                        bot.send_message(msg.chat.id, "Subscribed to channel")
                            .await?;
                    }
                    Err(e) => {
                        bot.send_message(msg.chat.id, format!("Error: {}", e))
                            .await?;
                    }
                }
            }
        }
    }

    Ok(())
}
