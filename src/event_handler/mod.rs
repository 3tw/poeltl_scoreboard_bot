use crate::constants;
use crate::messages::get::get_messages;
use crate::messages::process::get_leaderboard;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let message_intro: String = String::from(constants::MESSAGE_INTRO);
        let mentioned = msg.mentions.iter().any(|user| user.id == constants::APP_ID);

        if msg.content == constants::COMMAND || mentioned {
            // let channel_id = msg.channel_id.0;
            let http = &ctx.http;
            let messages: Vec<Message> = get_messages(http, msg.channel_id.0).await;
            println!("messages: ");
            println!("{:#?}", &messages.len());
            let leaderboard: ()= get_leaderboard(messages);

            let message = message_intro.clone();
            if let Err(e) = msg.channel_id.say(&ctx.http, message).await {
                println!("Error sending message: {:?}", e);
            }
        };
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
