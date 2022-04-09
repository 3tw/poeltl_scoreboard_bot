mod event_handler;
mod messages;
mod constants;

use crate::event_handler::Handler;
use serenity::{prelude::*};
use std::env;

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Failed to create a client");

    if let Err(e) = client.start().await {
        println!("Client error: {:?}", e);
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
// }
