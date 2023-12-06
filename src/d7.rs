use crate::{read_file, PuzzleRun};
use itertools::Itertools;
use std::cmp::Ordering;

pub(crate) fn get_runs() -> Vec<Box<dyn PuzzleRun>> {
    vec![Box::new(Part1)]
}

#[derive(Debug, PartialOrd, PartialEq)]
enum Score {
    High,
    Pair,
    TwoPair,
    Three,
    Full,
    Four,
    Five,
}
#[derive(Debug)]
struct Hand {
    cards: [u8; 5],
    sorted_cards: [u8; 5],
    bid: u32,
    score: Score,
}

impl Hand {
    fn new(cards: [u8; 5], bid: u32) -> Self {
        let mut sorted_cards = cards.clone();
        sorted_cards.sort();
        Self {
            cards,
            sorted_cards,
            bid,
            score: Self::score(&sorted_cards),
        }
    }

    fn has_seq(cards: &[u8; 5], len: usize, start: usize) -> Option<usize> {
        for i in (start..5 - len + 1) {
            if cards[i..(i + len)].iter().all_equal() {
                return Some(i + len);
            }
        }
        None
    }

    // must be sorted
    fn score(cards: &[u8; 5]) -> Score {
        if Hand::has_seq(&cards, 5, 0).is_some() {
            Score::Five
        } else if Hand::has_seq(&cards, 4, 0).is_some() {
            Score::Four
        } else if let Some(i) = Hand::has_seq(&cards, 3, 0) {
            if i == 0 && Hand::has_seq(&cards, 2, 2).is_some() {
                Score::Full
            } else if i == 2 && Hand::has_seq(&cards, 2, 0).is_some() {
                Score::Full
            } else {
                Score::Three
            }
        } else if let Some(i) = Hand::has_seq(&cards, 2, 0) {
            if Hand::has_seq(&cards, 2, i + 1).is_some() {
                Score::TwoPair
            } else {
                Score::Pair
            }
        } else {
            Score::High
        }
    }
}

fn cmp_card(s: u8, o: u8) -> Ordering {
    if s == o {
        Ordering::Equal
    } else if s == b'A' {
        Ordering::Greater
    } else if o == b'A' {
        Ordering::Less
    } else if s == b'K' {
        Ordering::Greater
    } else if o == b'K' {
        Ordering::Less
    } else if s == b'Q' {
        Ordering::Greater
    } else if o == b'Q' {
        Ordering::Less
    } else if s == b'J' {
        Ordering::Greater
    } else if o == b'J' {
        Ordering::Less
    } else if s == b'T' {
        Ordering::Greater
    } else if o == b'T' {
        Ordering::Less
    } else {
        s.cmp(&o)
    }
}

impl Eq for Hand {}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.sorted_cards == other.sorted_cards
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.sorted_cards == other.sorted_cards {
            Some(Ordering::Equal)
        } else {
            match self.score.partial_cmp(&other.score) {
                Some(Ordering::Equal) => {
                    for (s, o) in self.cards.iter().zip(other.cards.iter()) {
                        match cmp_card(*s, *o) {
                            Ordering::Equal => continue,
                            ord => return Some(ord),
                        }
                    }
                    panic!()
                }
                ord => return ord,
            }
        }
    }
}

fn parse_hand(input: &str) -> Hand {
    let input = input.trim();
    Hand::new(
        input.as_bytes()[0..5].try_into().unwrap(),
        input[6..].parse().unwrap(),
    )
}
struct Part1;

impl PuzzleRun for Part1 {
    fn input_data(&self) -> anyhow::Result<&str> {
        //read_file("input/day7.txt")

        Ok("32T3K 765
               T55J5 684
               KK677 28
               KTJJT 220
               QQQJA 483")
    }

    fn run(&self, input: &str) -> String {
        let mut hands: Vec<Hand> = input.lines().map(|s| parse_hand(s)).collect();
        hands.sort();

        for (i, h) in hands.iter().enumerate() {
            println!(
                "{} ({:?}):\t{} [{}] [bid: {}] [score: {}]",
                i + 1,
                h.score,
                std::str::from_utf8(&h.sorted_cards).unwrap(),
                std::str::from_utf8(&h.cards).unwrap(),
                h.bid,
                h.bid as usize * (i + 1),
            );
        }
        format!(
            "{}",
            hands
                .into_iter()
                .enumerate()
                //                .inspect(|v| println!("{:?}", v))
                .fold(0, |acc, (i, hand)| acc + (i + 1) * hand.bid as usize)
        )
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_score() {
        let h = "55555 999";
        let h = parse_hand(h);
        assert!(matches!(h.score, Score::Five));

        let h = "55455 999";
        let h = parse_hand(h);
        assert!(matches!(h.score, Score::Four));

        let h = "54455 999";
        let h = parse_hand(h);
        assert!(matches!(h.score, Score::Full));

        let h = "54655 999";
        let h = parse_hand(h);
        assert!(matches!(h.score, Score::Three));

        let h = "5445K 999";
        let h = parse_hand(h);
        assert!(matches!(h.score, Score::TwoPair));

        let h = "544AK 999";
        let h = parse_hand(h);
        assert!(matches!(h.score, Score::Pair));

        let h = "534AK 999";
        let h = parse_hand(h);
        assert!(matches!(h.score, Score::High));

        let h = "24443 999";
        let h = parse_hand(h);
        assert!(matches!(h.score, Score::Three));

        let h = "KK677 999";
        let h = parse_hand(h);
        assert!(matches!(h.score, Score::TwoPair));
    }

    #[test]
    fn test_order() {
        let h1 = "55555 999";
        let h1 = parse_hand(h1);
        let h2 = "55554 999";
        let h2 = parse_hand(h2);

        assert!(h1 > h2);

        let h1 = "33332 999";
        let h1 = parse_hand(h1);
        let h2 = "2AAAA 999";
        let h2 = parse_hand(h2);

        assert!(h1 > h2);

        let h1 = "23456 999";
        let h1 = parse_hand(h1);
        let h2 = "65432 999";
        let h2 = parse_hand(h2);

        assert!(h1 == h2);

        let h1 = "3A3A3 999";
        let h1 = parse_hand(h1);
        let h2 = "AA432 999";
        let h2 = parse_hand(h2);

        assert!(h1 > h2);
    }

    #[test]
    fn test_part1() {
        println!("{}", Part1.run(Part1.input_data().unwrap()));
    }
}
