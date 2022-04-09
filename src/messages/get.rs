use chrono::{Datelike, Local};
use serenity::{
    http::Http,
    model::{channel::Message, id::ChannelId},
};
use std::sync::Arc;

pub async fn get_messages(http: &Arc<Http>, channel_id: u64) -> Vec<Message> {
    let mut total: Vec<Message> = Vec::new();

    // Get recent messages
    let recent = ChannelId(channel_id)
        .messages(http, |g| g.limit(100))
        .await
        .unwrap();
    total.extend(recent);

    let current_month: u32 = Local::now().month();
    let mut oldest_message: &Message = total.last().unwrap();

    while oldest_message.timestamp.month() >= current_month {
      let previous_oldest_id = total.last().unwrap().id.0;
        let new_batch = ChannelId(channel_id)
            .messages(http, |retriever| {
                retriever.before(oldest_message.id.0).limit(100)
            })
            .await
            .unwrap();

        
        total.extend(new_batch);
        oldest_message = total.last().unwrap();
        
        if previous_oldest_id == oldest_message.id.0 {
          println!("BREAK");
          break;
        }
    }

    total
}