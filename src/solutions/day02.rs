use std::fmt;
const WIN: i32 = 6;
const DRAW: i32 = 3;
const LOSE: i32 = 0;

pub fn part_one(input: String) -> i32 {
    let mut rounds =  Vec::<RockPaperScissorsRound>::new();

    input.lines().for_each(|f| {
        let split : Vec<String> = f.split_whitespace()
            .map(|s| s.to_string())
            .collect();

        
        let first_char = split[0].chars().nth(0).unwrap();
        let second_char = split[1].chars().nth(0).unwrap();

        rounds.push(RockPaperScissorsRound {
            players_move: parse_player_symbol(second_char),
            opponents_move: parse_opponent_symbol(first_char)
        });
    });

    return rounds.iter().map(|s: &RockPaperScissorsRound| s.score_round()).sum::<i32>();
}

pub fn part_two(input: String) -> i32 {
    let mut rounds =  Vec::<RockPaperScissorsRound>::new();


    input.lines().for_each(|f| {
        let split : Vec<String> = f.split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let first_char = split[0].chars().nth(0).unwrap();
        let second_char = split[1].chars().nth(0).unwrap();

        let game_status = parse_game_status(second_char);
        let opponent_move = parse_opponent_symbol(first_char);
        let player_move = calculate_player_move(game_status, opponent_move);

        rounds.push(RockPaperScissorsRound {
            players_move: player_move,
            opponents_move: opponent_move
        });
    });

    return rounds.iter().map(|s: &RockPaperScissorsRound| s.score_round()).sum::<i32>();
}

fn parse_player_symbol(players_move: char) -> Symbols {
    return match players_move {
        'X' => Symbols::Rock,
        'Y' => Symbols::Paper,
        'Z' => Symbols::Scissors,
        _=> panic!("Not covered")
    };
}

fn parse_opponent_symbol(opponent_move: char) -> Symbols {
    return match opponent_move {
        'A' => Symbols::Rock,
        'B' => Symbols::Paper,
        'C' => Symbols::Scissors,
        _=> panic!("Not covered")
    };
}

fn parse_game_status(game_status_symbol: char) -> GameStatus {
    return match game_status_symbol {
        'X' => GameStatus::Lose,
        'Y' => GameStatus::Draw,
        'Z' => GameStatus::Win,
        _=> panic!("Not covered")
    };
}

fn calculate_player_move(game_status: GameStatus, opponent_move: Symbols) -> Symbols {
    match game_status {
        GameStatus::Draw => return opponent_move,
        GameStatus::Win => return opponent_move.loses_to(),
        GameStatus::Lose => return opponent_move.beats()
    };
}

enum GameStatus {
    Draw,
    Win,
    Lose
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Symbols {
    Rock,
    Paper,
    Scissors
}

impl fmt::Display for Symbols {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Symbols::Rock => write!(f, "Rock"),
            Symbols::Paper => write!(f, "Paper"),
            Symbols::Scissors => write!(f, "Scissors")
        }
    }
}

impl Symbols {
    fn loses_to(&self) -> Symbols {
        match self {
            Symbols::Paper => Symbols::Scissors,
            Symbols::Rock => Symbols::Paper,
            Symbols::Scissors => Symbols::Rock
        }
    }

    fn beats(&self) -> Symbols {
        match self {
            Symbols::Paper => Symbols::Rock,
            Symbols::Rock => Symbols::Scissors,
            Symbols::Scissors => Symbols::Paper
            
        }
    }

    fn innate_value(&self) -> i32 {
        match self {
            Symbols::Paper => 2,
            Symbols::Rock => 1,
            Symbols::Scissors => 3    
        }
    }
    
}

struct RockPaperScissorsRound {
    opponents_move: Symbols,
    players_move: Symbols
}


impl RockPaperScissorsRound {
    fn score_round(&self) -> i32 {
        let mut total = 0;

        total = total + self.players_move.innate_value();

        if &self.players_move.beats() == &self.opponents_move {
            total = total + WIN;
        }
        if &self.players_move.loses_to() == &self.opponents_move {
            total = total + LOSE;
        }
        if &self.players_move == &self.opponents_move {
            total = total + DRAW;
        }

        return total;
    }
}