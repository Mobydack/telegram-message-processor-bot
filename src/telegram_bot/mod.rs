use crate::configuration::Configuration;
use log;
use teloxide::prelude::*;
use teloxide::types::MessageKind;

async fn is_mention_filter(bot: &Bot, message: &Message) -> bool {
    if let Some(username) = bot.get_me().await.ok().and_then(|user| user.user.username) {
        if let Some(text) = message.text() {
            text.contains(format!("@{username}").as_str())
        } else {
            false
        }
    } else {
        log::error!("Bot doesn't have username.");

        false
    }
}

pub async fn create(configuration: &Configuration) {
    let bot = Bot::new(configuration.telegram.token.clone());

    log::info!("Starting telegram bot...");

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        if !is_mention_filter(&bot, &msg).await {
            return Ok(());
        }

        if let MessageKind::Common(common_msg) = &msg.kind {
            if let Some(replied_message) = &common_msg.reply_to_message {
                log::info!("{:?}", replied_message);

                let mut normalized_msg = String::new();

                if let Some(replied_message_text) = replied_message.text() {
                    normalized_msg += format!("{replied_message_text}\n").as_str();
                }

                if let Some(caption) = replied_message.caption() {
                    normalized_msg += caption;
                }

                if !normalized_msg.is_empty() {
                    bot.send_message(msg.chat.id, normalized_msg)
                        .reply_to_message_id(replied_message.id)
                        .await?;
                }
            }
        }

        Ok(())
    })
    .await
}
