use crate::configuration::telegram::TelegramConfiguration;
use crate::model::types::ModelAPI;
use log;
use std::rc::Rc;
use teloxide::prelude::*;
use teloxide::types::{Message, ParseMode};
use teloxide::utils::html::escape;

#[derive(Debug, Clone)]
struct OriginalMessage(Message);

pub async fn create(telegram_configuration: &TelegramConfiguration, _model: Rc<dyn ModelAPI>) {
    let bot = Bot::new(telegram_configuration.token.clone());

    log::info!("Starting telegram bot...");

    Dispatcher::builder(
        bot,
        Update::filter_message().branch(
            dptree::filter_async(|bot: Bot, msg: Message| async move {
                match bot.get_me().await.ok().and_then(|user| user.user.username) {
                    Some(username) => {
                        if let Some(text) = msg.text() {
                            text.contains(format!("@{username}").as_str())
                        } else {
                            false
                        }
                    }
                    _ => {
                        log::error!("Bot doesn't have username.");

                        false
                    }
                }
            })
            .chain(dptree::filter_map(|msg: Message| {
                Some(OriginalMessage(msg))
            }))
            .chain(Message::filter_reply_to_message())
            .chain(dptree::filter_map(|msg: Message| {
                match (msg.text(), msg.caption()) {
                    (None, None) => None,
                    (Some(text), None) | (None, Some(text)) => Some(text.to_string()),
                    (Some(text), Some(caption)) => Some(format!("{text}\n{caption}")),
                }
            }))
            .endpoint(
                |bot: Bot, raw_content: String, original_message: OriginalMessage| async move {
                    let content = escape(raw_content.as_str());

                    bot.send_message(original_message.0.chat.id, content)
                        .reply_to_message_id(original_message.0.id)
                        .parse_mode(ParseMode::Html)
                        .await?;

                    respond(())
                },
            ),
        ),
    )
    .default_handler(|_| async move {})
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}
