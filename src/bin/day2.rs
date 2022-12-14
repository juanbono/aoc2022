use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

#[derive(Debug, PartialEq, Clone, Copy)]
enum GameResult {
    Win,
    Lose,
    Draw,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum GameElement {
    Rock,
    Scissors,
    Paper,
}

impl GameElement {
    /// Returns true if self wins to other, false otherwise
    pub fn play(self, other: GameElement) -> GameResult {
        use GameElement::*;
        use GameResult::*;
        match (self, other) {
            (Rock, Scissors) => Win,
            (Scissors, Paper) => Win,
            (Paper, Rock) => Win,
            (elem1, elem2) if elem1 == elem2 => Draw,
            _ => Lose,
        }
    }

    pub fn score(&self) -> u32 {
        match self {
            GameElement::Rock => 1,
            GameElement::Paper => 2,
            GameElement::Scissors => 3,
        }
    }

    pub fn parse(letter: &str) -> Option<GameElement> {
        match letter {
            "A" | "X" => Some(GameElement::Rock),
            "B" | "Y" => Some(GameElement::Paper),
            "C" | "Z" => Some(GameElement::Scissors),
            _ => None,
        }
    }

    fn player_should_play(&self, input: &str) -> GameElement {
        let expected_outcome = match input {
            "X" => GameResult::Lose,
            "Y" => GameResult::Draw,
            "Z" => GameResult::Win,
            _ => unreachable!(),
        };

        self.element_for_result(expected_outcome)
    }

    fn element_for_result(&self, expected_outcome: GameResult) -> GameElement {
        match (self, expected_outcome) {
            (GameElement::Rock, GameResult::Win) => GameElement::Paper,
            (GameElement::Rock, GameResult::Lose) => GameElement::Scissors,
            (GameElement::Scissors, GameResult::Win) => GameElement::Rock,
            (GameElement::Scissors, GameResult::Lose) => GameElement::Paper,
            (GameElement::Paper, GameResult::Win) => GameElement::Scissors,
            (GameElement::Paper, GameResult::Lose) => GameElement::Rock,
            (element, GameResult::Draw) => element.to_owned(),
        }
    }
}

struct Tournament(Vec<(GameElement, GameElement)>);

impl Tournament {
    fn from_reader(reader: impl BufRead) -> Tournament {
        let mut games: Vec<(GameElement, GameElement)> = Vec::new();
        for line in reader.lines() {
            let line = line.unwrap();
            let mut game: Vec<GameElement> = line
                .split(" ")
                .map(|s| GameElement::parse(s).unwrap())
                .collect();
            game.reverse();
            games.push((game[0], game[1]));
        }
        Tournament(games)
    }

    pub fn from_reader_part2(reader: impl BufRead) -> Tournament {
        let mut games: Vec<(GameElement, GameElement)> = vec![];
        for line in reader.lines() {
            let line = line.unwrap();
            let line_input: Vec<&str> = line.split(" ").collect_vec();
            dbg!(&line_input);
            let oponent = GameElement::parse(line_input[0]).unwrap();
            let game = (oponent.player_should_play(line_input[1]), oponent);
            dbg!(&game);
            games.push(game)
        }

        Tournament(games)
    }

    pub fn calculate_score(&self) -> u32 {
        let mut score = 0;

        for (player, oponent) in self.0.clone() {
            match player.play(oponent) {
                GameResult::Win => score += player.score() + 6,
                GameResult::Lose => score += player.score() + 0,
                GameResult::Draw => score += player.score() + 3,
            }
        }

        score
    }
}

fn main() {
    let input_file = File::open("inputs/input2").unwrap();
    let reader = BufReader::new(input_file);
    let tournament = Tournament::from_reader_part2(reader);

    let score = tournament.calculate_score();

    println!("The score is {score}");
}


#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use crate::Tournament;

    #[test]
    fn part2() {
        let input = "A Y\nB X\nC Z";
        let reader = BufReader::new(input.as_bytes());
        let tournament = Tournament::from_reader_part2(reader);

        assert_eq!(12, tournament.calculate_score());
    }
}