mod core;
mod db;
mod scheduler;
mod telegram_bot;
mod twitch_api;
mod yt_api;

#[macro_use]
extern crate failure;

use crate::core::bot::{Bot, TelegramBot};
use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fmt::format;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    TelegramBot::start().await.expect("TODO: panic message");
    Ok(())
}
