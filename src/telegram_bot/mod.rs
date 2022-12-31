use std::env;
use std::sync::{Arc, Mutex};

use anyhow::Result;
use diesel::{Connection, PgConnection};
use log::warn;
use teloxide::dispatching::DefaultKey;
use teloxide::{prelude::*, utils::command::BotCommands};

use crate::telegram_bot::command_handler::{answer, Command};

mod command_handler;

struct Db {
    conn: Arc<Mutex<PgConnection>>,
}

pub async fn start_bot() -> Result<()> {
    let bot = teloxide::Bot::new("1803593210:AAENaqqyn54XbCF0lHJm_FX0gJvQDfT7vyc");
    bot.set_my_commands(Command::bot_commands())
        .await
        .expect("TODO: panic message");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut pg = PgConnection::establish(&database_url).unwrap();
    let db = Db {
        conn: Arc::new(Mutex::new(pg)),
    };
    let handler = Update::filter_message()
        .filter_command::<Command>()
        .branch(dptree::endpoint(answer));

    let dispatcher = Dispatcher::builder(bot.clone(), handler)
        .dependencies(dptree::deps![db])
        .enable_ctrlc_handler()
        .default_handler(|upd| async move {
            warn!("Unknown update: {:?}", upd);
        })
        .error_handler(LoggingErrorHandler::with_custom_text(
            "An error occured while handling an update",
        ))
        .build()
        .dispatch()
        .await;
    Ok(())
}
