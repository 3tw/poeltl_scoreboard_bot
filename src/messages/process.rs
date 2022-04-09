use serenity::model::{channel::Message, id::MessageId, user::User};

struct Player {
    id: u64,
    name: String,
    points: u8,
}

pub fn get_leaderboard(messages: Vec<Message>) -> () {
    let filtered_messages:Vec<&Message> = messages.iter().filter(|msg| has_score(&msg.content)).collect();
    println!("{:#?}",filtered_messages.len());
    ()
}

fn has_score(msg: &str) -> bool {
    let content: &str = &msg.to_lowercase();
    if content.starts_with("poeltl") && content.contains("/8") {
        return true;
    }
    return false;
}
