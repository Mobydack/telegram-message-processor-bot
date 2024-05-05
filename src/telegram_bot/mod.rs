use crate::configuration::telegram::TelegramConfiguration;
use crate::model::types::ModelAPI;
use log;
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::types::{Message, ParseMode};
use teloxide::utils::html::escape;

#[derive(Debug, Clone)]
struct Parameters {
    model_api: Arc<dyn ModelAPI>,
}

pub async fn create(telegram_configuration: &TelegramConfiguration, model: Arc<dyn ModelAPI>) {
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
                if let Some(replied_to_message) = msg.reply_to_message() {
                    match (replied_to_message.text(), replied_to_message.caption()) {
                        (None, None) => None,
                        (Some(text), None) | (None, Some(text)) => Some(text.to_string()),
                        (Some(text), Some(caption)) => Some(format!("{text}\n{caption}")),
                    }
                } else {
                    None
                }
            }))
            .endpoint(
                |bot: Bot,
                 raw_content: String,
                 msg: Message,
                 params: Parameters| async move {
                    log::info!("Message: {:?}", msg.text());
                    log::info!("Original message: {:?}", msg.text());

                    if let Some(replied_to_message) = msg.reply_to_message() {
                        let result = params
                            .model_api
                            .get_alternative(String::from(
                                "As an advanced language model, your task is to process user messages and transform them into a normalized format that adheres to the system's restrictions. Analyze the original message, identify its core meaning, and condense it while maintaining the essential information. Bypass any limitations that may hinder the message's transmission by rephrasing and restructuring the content appropriately. Ensure that the resulting message is concise, coherent, and correctly formatted according to the system's guidelines. Highlight the main points and present them in a clear and easily understandable manner. Your output should be a normalized message that effectively conveys the user's intended meaning while working within the given constraints."),
                                             raw_content
                            )
                            .await
                            .unwrap();
                        let content = escape(result.as_str());

                        bot.send_message(msg.chat.id, content)
                            .reply_to_message_id(replied_to_message.id)
                            .parse_mode(ParseMode::Html)
                            .await?;
                    }

                    respond(())
                },
            ),
        ),
    )
    .dependencies(dptree::deps![Parameters { model_api: model }])
    .default_handler(|_| async move {})
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}
