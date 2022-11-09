use std::error::Error;

use csv::Writer;
use hashbrown::HashMap;
use serde::Serialize;
use skillratings::{
    trueskill::{expected_score, get_rank, trueskill, TrueSkillConfig, TrueSkillRating},
    Outcomes,
};

#[derive(Debug, Clone)]
struct Player {
    id: String,
    name: String,
    wins: u32,
    losses: u32,
    rating: TrueSkillRating,
}

#[derive(Debug, Clone)]
struct Match {
    player1_id: String,
    player2_id: String,
    outcome: Outcomes,
}

#[derive(Debug, Clone, Serialize)]
struct CsvRow {
    place: usize,
    name: String,
    rating: f64,
    wins: u32,
    losses: u32,
    id: String,
}

/// Gets all players found in the database as a Hashmap containing their ID as the key and the player as the value.
fn get_all_players() -> Result<HashMap<String, Player>, Box<dyn Error>> {
    let conn = rusqlite::Connection::open("./data/ultimate_player_database.db")?;

    let mut stmt = conn.prepare("SELECT player_id, tag FROM players")?;

    let mut players = HashMap::new();

    let player_iter = stmt.query_map([], |row| {
        Ok(Player {
            id: row.get(0).unwrap_or_else(|_| "0".to_string()),
            name: row.get(1)?,
            wins: 0,
            losses: 0,
            rating: TrueSkillRating::new(),
        })
    })?;

    for player in player_iter {
        let player = player?;
        let id = player.id.to_owned();

        players.insert(id, player);
    }

    Ok(players)
}

/// Gets all matches found in the database as a vector of matches.
fn get_all_matches() -> Result<Vec<Match>, Box<dyn Error>> {
    let conn = rusqlite::Connection::open("./data/ultimate_player_database.db")?;

    let mut stmt = conn.prepare("SELECT p1_id, p2_id, p1_score, p2_score FROM sets")?;

    let mut matches = Vec::new();

    let match_iter = stmt.query_map([], |row| {
        let p1_id: String = row.get(0).unwrap_or_else(|_| "0".to_string());
        let p2_id: String = row.get(1).unwrap_or_else(|_| "0".to_string());
        let p1_score: i32 = row.get(2)?;
        let p2_score: i32 = row.get(3)?;

        let outcome = match p1_score - p2_score {
            x if x > 0 => Outcomes::WIN,
            x if x < 0 => Outcomes::LOSS,
            _ => Outcomes::DRAW,
        };

        Ok(Match {
            player1_id: p1_id,
            player2_id: p2_id,
            outcome,
        })
    })?;

    for m in match_iter {
        let ma = m?;

        if ma.player1_id != "0" && ma.player2_id != "0" {
            matches.push(ma);
        }
    }

    Ok(matches)
}

/// Simulates the match and updates the players with the new TrueSkill ratings.
fn simulate_match(player1: &mut Player, player2: &mut Player, outcome: Outcomes) {
    let config = TrueSkillConfig::new();

    let (p1, p2) = trueskill(&player1.rating, &player2.rating, &outcome, &config);

    player1.rating = p1;
    player2.rating = p2;

    match outcome {
        Outcomes::WIN => {
            player1.wins += 1;
            player2.losses += 1;
        }
        Outcomes::LOSS => {
            player1.losses += 1;
            player2.wins += 1;
        }
        Outcomes::DRAW => {}
    }
}

#[allow(dead_code)]
/// Predicts the outcome of a match between two players.
fn predict_game(player_list: &HashMap<String, Player>) {
    let config = TrueSkillConfig::new();

    // Just 2 example players, you can change these to whatever you want.
    // You can look up the ID of your desired players in the results.csv file, last column.
    if let Some(mkleo) = player_list.get("222927") {
        if let Some(dabuz) = player_list.get("4702") {
            println!(
                "Mkleo vs Dabuz: {:?}",
                expected_score(&mkleo.rating, &dabuz.rating, &config)
            );
        }
    };
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut player_hashmap = get_all_players()?;

    println!("Player count: {}", player_hashmap.len());

    let match_vec = get_all_matches()?;

    println!("Match count: {}", match_vec.len());

    for (i, m) in match_vec.iter().enumerate() {
        if let Some([p1, p2]) = player_hashmap.get_many_mut([&m.player1_id, &m.player2_id]) {
            simulate_match(p1, p2, m.outcome);
        }

        // Just letting the user know that progress is being made.
        // This might take a while to run (~10s in release mode on my PC, ~50s in debug).
        if i % 100_000 == 0 {
            println!("Match {} of {}", i, match_vec.len());
        }
    }

    // Converting the hashmap into a vector of players and sorting it by display rank.
    let mut player_vec: Vec<&Player> = player_hashmap.values().collect();

    player_vec.sort_by(|a, b| {
        get_rank(&b.rating)
            .partial_cmp(&get_rank(&a.rating))
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut wtr = Writer::from_path("./data/results.csv")?;

    for (i, p) in player_vec.iter().enumerate() {
        wtr.serialize(CsvRow {
            place: i + 1,
            name: p.name.to_owned(),
            rating: get_rank(&p.rating),
            wins: p.wins,
            losses: p.losses,
            id: p.id.to_owned(),
        })?;
    }

    predict_game(&player_hashmap);

    Ok(())
}
