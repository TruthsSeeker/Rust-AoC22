use std::{path::Path, fs::File, io::{self, Read}};

fn main() {
    let strategy = match read_file_to_str("data/strategy.txt") {
        Ok(s) => s,
        Err(e) => panic!("Error reading file: {}", e),
    };
    let turns = strategy.split("\n");
    
    let mut score1 = 0;
    let mut score2 = 0;
    for turn in turns {
        println!("Turn: {}", turn);

        let moves: Vec<&str> = turn.split(" ").collect();

        let player_move = get_move(moves[1]);
        println!("Player move: {:?}", &player_move);
        let opponent_move = get_move(moves[0]);
        println!("Opponent move: {:?}", opponent_move);

        let outcome = get_result(&player_move, &opponent_move);
        println!("Outcome: {:?}", outcome);

        score1 += get_score(player_move, outcome);

        let required_outcome = get_required_outcome(moves[1]);
        println!("Required outcome: {:?}", required_outcome);

        let required_move = get_required_move(&opponent_move, &required_outcome);
        println!("Required move: {:?}", required_move);

        score2 += get_score(required_move, required_outcome);

    }
    println!("Score first part: {}", score1);
    println!("Score second part: {}", score2);
    
}

#[derive(Debug)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug)]
enum Outcome {
    Win = 6,
    Lose = 0,
    Tie = 3,
}

fn get_result(player_move: &Move, opponent_move: &Move) -> Outcome {
    let result = match (player_move, opponent_move) {
        (Move::Rock, Move::Rock) => Outcome::Tie,
        (Move::Rock, Move::Paper) => Outcome::Lose,
        (Move::Rock, Move::Scissors) => Outcome::Win,
        (Move::Paper, Move::Rock) => Outcome::Win,
        (Move::Paper, Move::Paper) => Outcome::Tie,
        (Move::Paper, Move::Scissors) => Outcome::Lose,
        (Move::Scissors, Move::Rock) => Outcome::Lose,
        (Move::Scissors, Move::Paper) => Outcome::Win,
        (Move::Scissors, Move::Scissors) => Outcome::Tie,
    };
    result
}

fn get_score(player_move: Move, outcome: Outcome) -> i32 {
    player_move as i32 + outcome as i32
}

fn get_move(m: &str) -> Move {
    match m {
        "A" | "X" => Move::Rock,
        "B" | "Y"=> Move::Paper,
        "C" | "Z" => Move::Scissors,
        _ => panic!("Invalid move: {}", m),
    }
}

fn get_required_outcome(o: &str) -> Outcome {
    match o {
        "X" => Outcome::Lose,
        "Y" => Outcome::Tie,
        "Z" => Outcome::Win,
        _ => panic!("Invalid outcome: {}", o),
    }
}

fn get_required_move(opponent_move: &Move, required_outcome: &Outcome) -> Move {
    match (opponent_move, required_outcome) {
        (Move::Rock, Outcome::Lose) => Move::Scissors,
        (Move::Rock, Outcome::Tie) => Move::Rock,
        (Move::Rock, Outcome::Win) => Move::Paper,
        (Move::Paper, Outcome::Lose) => Move::Rock,
        (Move::Paper, Outcome::Tie) => Move::Paper,
        (Move::Paper, Outcome::Win) => Move::Scissors,
        (Move::Scissors, Outcome::Lose) => Move::Paper,
        (Move::Scissors, Outcome::Tie) => Move::Scissors,
        (Move::Scissors, Outcome::Win) => Move::Rock,
    }
}

fn read_file_to_str(path_str: &str) -> Result<String, io::Error> {
    let path = Path::new(path_str);
    println!("Path to file: {}", path.display());
    let mut file = File::open(path_str)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}