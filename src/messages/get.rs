use serenity::{
  model::{channel::Message},
  prelude::*,
};

pub async fn get_messages(ctx: &Context, channel_id: u64) -> Result<Vec<Message>, SerenityError> {
  match ctx.http.get_messages(channel_id, "?limit=100").await {
      Ok(data) => Ok(data),
      Err(e) => Err(e),
  }
}