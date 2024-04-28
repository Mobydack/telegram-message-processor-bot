use crate::configuration::Configuration;
use log;
use teloxide::prelude::*;
use teloxide::types::{MessageEntityKind, MessageKind};

pub async fn create(configuration: &Configuration) {
    let bot = Bot::new(configuration.telegram.token.clone());

    log::info!("Starting telegram bot...");

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        if let MessageKind::Common(common_msg) = &msg.kind {
            if let Some(text) = msg.text() {
                if let Some(replied_message) = &common_msg.reply_to_message {
                    if let Some(entities) = msg.entities() {
                        let bot_username = bot
                            .get_me()
                            .await?
                            .user
                            .username
                            .expect("Bot doesn't have username.");

                        if let Some(_) = entities.iter().find(|entity| {
                            if entity.kind != MessageEntityKind::Mention {
                                return false;
                            }

                            if &text[entity.offset..(entity.offset + entity.length)]
                                != format!("@{}", bot_username)
                            {
                                return false;
                            }

                            true
                        }) {
                            bot.send_message(msg.chat.id, "Hello, world!")
                                .reply_to_message_id(replied_message.id)
                                .await?;
                        }
                    }
                }
            }
        }

        Ok(())
    })
    .await
}
