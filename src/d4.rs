use crate::{read_file, PuzzleRun};
use std::collections::VecDeque;

use regex::Regex;

pub(crate) fn get_runs() -> Vec<Box<dyn PuzzleRun>> {
    vec![Box::new(Part1)]
}

#[derive(Debug)]
struct Card {
    id: u32,
    winners: Vec<u32>,
    cards: Vec<u32>,
}

impl Card {
    fn new(id: u32, mut winners: Vec<u32>, mut cards: Vec<u32>) -> Self {
        winners.sort();
        Self { id, winners, cards }
    }

    fn count_winners(&self) -> u32 {
        self.cards
            .iter()
            .filter(|c| self.winners.binary_search(&c).is_ok())
            .count()
            .try_into()
            .unwrap()
    }
}

fn to_vec(s: &str) -> Vec<u32> {
    s.trim()
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}
impl std::str::FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"Card\s+(\d+): ([\s\w]+)\|([\s\w]+)").unwrap();
        match re.captures(s).map(|c| c.extract()) {
            Some((_, [card, win_str, card_str])) => Ok(Card::new(
                card.parse::<u32>().unwrap(),
                to_vec(win_str),
                to_vec(card_str),
            )),
            None => Err(format!("failed to parse line: {}", s)),
        }
    }
}
struct Part1;

impl PuzzleRun for Part1 {
    fn input_data(&self) -> anyhow::Result<&str> {
        read_file("input/day4.txt")
        /*
        Ok("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11")
        */
    }

    fn run(&self, input: &str) -> String {
        let cards: Vec<Card> = input.lines().map(|s| s.parse().unwrap()).collect();
        let score: u32 = cards
            .iter()
            .map(|c| c.count_winners())
            .filter(|c| *c > 0)
            .map(|winners| 2_u32.pow(winners - 1))
            .sum();

        format!("{}", score)
    }
}

struct Part2;

impl PuzzleRun for Part2 {
    fn input_data(&self) -> anyhow::Result<&str> {
        read_file("input/day4.txt")
        /*
        Ok("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11")
        */
    }

    fn run(&self, input: &str) -> String {
        let cards: Vec<Card> = input.lines().map(|s| s.parse().unwrap()).collect();
        let card_winners: Vec<(usize, u32)> = cards
            .iter()
            .enumerate()
            .map(|(i, c)| (i, c.count_winners()))
            .collect();

        let mut q: VecDeque<(usize, u32)> = card_winners.iter().copied().collect();
        let mut card_count: u32 = 0;

        while !q.is_empty() {
            card_count += 1;
            let (card, mut winners) = q.pop_front().unwrap();
            while winners > 0 {
                q.push_back(card_winners[card + winners as usize]);
                winners -= 1;
            }
        }
        format!("{}", card_count)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_parse() {
        let s = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let re = Regex::new(r"Card\s+(\d+): ([\s\w]+)\|([\s\w]+)").unwrap();

        assert!(re.is_match(s));
        /*
        let c: Card = s.parse().unwrap();
        assert_eq!(c.id, 1);
        assert_eq!(c.winners.len(), 5);
        assert_eq!(c.winners, vec![41, 48, 83, 86, 17]);
        */
    }

    #[test]
    fn test_part1() {
        println!("{}", Part1.run(Part1.input_data().unwrap()))
    }

    #[test]
    fn test_part2() {
        println!("{}", Part2.run(Part2.input_data().unwrap()))
    }
}
