use serenity::model::channel::Message;

#[derive(Debug)]
struct Player {
    id: u64,
    name: String,
    score: u8,
    guesses: u8,
}

impl Player {
    fn new(id: u64, name: String) -> Player {
        Player {
            id: id,
            name: name,
            score: 0,
            guesses: 0,
        }
    }
}

pub fn get_leaderboard(messages: Vec<Message>) -> () {
    let filtered_messages: Vec<Message> = get_messages_with_keywords(messages);
    let mut players: Vec<Player> = get_players(filtered_messages);

    players.sort_by(|a, b| a.score.cmp(&b.score));
    println!("{:#?}", players);
    ()
}

fn get_messages_with_keywords<'a>(messages: Vec<Message>) -> Vec<Message> {
    messages
        .into_iter()
        .filter(|msg| has_score(&msg.content))
        .collect()
}

fn get_players(messages: Vec<Message>) -> Vec<Player> {
    let mut players: Vec<Player> = Vec::new();
    let mut indexes: Vec<u64> = Vec::new();

    for msg in messages.into_iter() {
        let player_id = msg.author.id.0;
        let mut index = indexes.iter().position(|id| id == &player_id);

        // Create Player
        if index == None {
            let player = Player::new(player_id, msg.author.name);
            index = Some(players.len());
            players.push(player);
            indexes.push(player_id);
        }

        // Update Player score
        let point_index: i8 = match msg.content.find("/8") {
            Some(i) => (i - 1) as i8,
            None => -1,
        };

        let point: u8 = if point_index > 0 {
            match msg.content.chars().nth(point_index as usize) {
                Some(c) => c as u8 - '0' as u8,
                None => 0,
            }
        } else {
            0
        };

        players[index.unwrap()].score += point;
        players[index.unwrap()].guesses += 1;
    }

    players
}

fn has_score(msg: &str) -> bool {
    let content: &str = &msg.to_lowercase();
    if content.starts_with("poeltl") && content.contains("/8") {
        return true;
    }
    return false;
}
