use crate::{read_file, PuzzleRun};
use itertools::Itertools;
use std::cmp::Ordering;

pub(crate) fn get_runs() -> Vec<Box<dyn PuzzleRun>> {
    vec![Box::new(Part1)]
}

trait Part {
    fn score(hand: &[u8]) -> (Score, Vec<u8>);
    fn hands_eq(this: &Hand<Self>, that: &Hand<Self>) -> bool;
    fn compare_hands(this: &Hand<Self>, that: &Hand<Self>) -> Option<Ordering>;
    fn cmp_card(s: u8, o: u8) -> Ordering;
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
struct Hand<P: ?Sized> {
    cards: [u8; 5],
    bid: u32,
    score: Score,
    sorted_cards: Vec<u8>,
    phantom: std::marker::PhantomData<P>,
}

impl<P: Part> Hand<P> {
    fn new(cards: [u8; 5], bid: u32, part: P) -> Self {
        let (score, sorted_cards) = P::score(&cards);
        Self {
            cards,
            bid,
            score,
            sorted_cards,
            phantom: std::marker::PhantomData,
        }
    }
}

// cards must be sorted
fn has_seq(cards: &[u8], len: usize, start: usize) -> Option<usize> {
    for i in (start..cards.len() - len + 1) {
        if cards[i..(i + len)].iter().all_equal() {
            return Some(i + len);
        }
    }
    None
}

impl Part for Part1 {
    // must be sorted
    fn score(cards: &[u8]) -> (Score, Vec<u8>) {
        let mut sorted: Vec<u8> = Default::default();
        cards.clone_into(&mut sorted);
        sorted.sort_by(|&s, &o| Self::cmp_card(s, o));

        if has_seq(&sorted, 5, 0).is_some() {
            (Score::Five, sorted)
        } else if has_seq(&sorted, 4, 0).is_some() {
            (Score::Four, sorted)
        } else if let Some(i) = has_seq(&sorted, 3, 0) {
            if i == 3 && has_seq(&sorted, 2, 3).is_some() {
                (Score::Full, sorted)
            } else if i == 5 && sorted[0] == sorted[1] {
                (Score::Full, sorted)
            } else {
                (Score::Three, sorted)
            }
        } else if let Some(i) = has_seq(&sorted, 2, 0) {
            if has_seq(&sorted, 2, i).is_some() {
                (Score::TwoPair, sorted)
            } else {
                (Score::Pair, sorted)
            }
        } else {
            (Score::High, sorted)
        }
    }

    fn hands_eq(this: &Hand<Self>, that: &Hand<Self>) -> bool {
        this.sorted_cards == that.sorted_cards
    }

    fn compare_hands(this: &Hand<Self>, that: &Hand<Self>) -> Option<Ordering> {
        let mut this_sorted = this.cards.clone();
        this_sorted.sort();
        let mut that_sorted = that.cards.clone();
        that_sorted.sort();

        if this_sorted == that_sorted {
            Some(Ordering::Equal)
        } else {
            match this.score.partial_cmp(&that.score) {
                Some(Ordering::Equal) => {
                    for (s, o) in this.cards.iter().zip(that.cards.iter()) {
                        match Self::cmp_card(*s, *o) {
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
}

impl<P: Part> Eq for Hand<P> {}

impl<P: Part> PartialEq for Hand<P> {
    fn eq(&self, other: &Self) -> bool {
        P::hands_eq(self, other)
    }
}

impl<P: Part> Ord for Hand<P> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<P: Part> PartialOrd for Hand<P> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        P::compare_hands(self, other)
    }
}

fn parse_hand<P: Part>(input: &str, part: P) -> Hand<P> {
    let input = input.trim();
    Hand::new(
        input.as_bytes()[0..5].try_into().unwrap(),
        input[6..].parse().unwrap(),
        part,
    )
}

struct Part1;

impl PuzzleRun for Part1 {
    fn input_data(&self) -> anyhow::Result<&str> {
        read_file("input/day7.txt")
        /*
        Ok("32T3K 765
               T55J5 684
               KK677 28
               KTJJT 220
               QQQJA 483")
        */
    }

    fn run(&self, input: &str) -> String {
        let mut hands: Vec<Hand<_>> = input.lines().map(|s| parse_hand(s, Self)).collect();
        hands.sort();

        /*
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
        */
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

#[derive(Debug)]
struct Part2;

impl Part for Part2 {
    // must be sorted
    fn score(cards: &[u8]) -> (Score, Vec<u8>) {
        let mut sorted: Vec<u8> = cards.iter().filter(|&c| *c != b'J').copied().collect();
        sorted.sort_by(|&s, &o| Self::cmp_card(s, o));
        let sorted = sorted; // no further mutability

        if sorted.len() == 5 {
            Part1::score(cards)
        } else if sorted.len() == 4 {
            if has_seq(&sorted, 4, 0).is_some() {
                (Score::Five, sorted)
            } else if has_seq(&sorted, 3, 0).is_some() {
                (Score::Four, sorted)
            } else if let Some(i) = has_seq(&sorted, 2, 0) {
                if has_seq(&sorted, 2, i).is_some() {
                    (Score::Full, sorted)
                } else {
                    (Score::Three, sorted)
                }
            } else {
                (Score::Pair, sorted)
            }
        } else if sorted.len() == 3 {
            // two wild cards
            if has_seq(&sorted, 3, 0).is_some() {
                (Score::Five, sorted)
            } else if has_seq(&sorted, 2, 0).is_some() {
                (Score::Four, sorted)
            } else {
                (Score::Three, sorted)
            }
        } else if sorted.len() == 2 {
            // three wild cards
            if has_seq(&sorted, 2, 0).is_some() {
                (Score::Five, sorted)
            } else {
                (Score::Four, sorted)
            }
        } else {
            (Score::Five, sorted)
        }
    }

    fn hands_eq(this: &Hand<Self>, that: &Hand<Self>) -> bool {
        this.sorted_cards == that.sorted_cards
    }

    fn compare_hands(this: &Hand<Self>, that: &Hand<Self>) -> Option<Ordering> {
        let mut this_sorted = this.cards.clone();
        this_sorted.sort();
        let mut that_sorted = that.cards.clone();
        that_sorted.sort();

        if this_sorted == that_sorted {
            Some(Ordering::Equal)
        } else {
            match this.score.partial_cmp(&that.score) {
                Some(Ordering::Equal) => {
                    for (s, o) in this.cards.iter().zip(that.cards.iter()) {
                        match Self::cmp_card(*s, *o) {
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
            Ordering::Less
        } else if o == b'J' {
            Ordering::Greater
        } else if s == b'T' {
            Ordering::Greater
        } else if o == b'T' {
            Ordering::Less
        } else {
            s.cmp(&o)
        }
    }
}

impl PuzzleRun for Part2 {
    fn input_data(&self) -> anyhow::Result<&str> {
        read_file("input/day7.txt")
        /*
        Ok("32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483")
        */
    }

    fn run(&self, input: &str) -> String {
        let mut hands: Vec<Hand<_>> = input.lines().map(|s| parse_hand(s, Self)).collect();
        hands.sort();

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
        let h = parse_hand(h, Part1);
        assert!(matches!(h.score, Score::Five));

        let h = "55455 999";
        let h = parse_hand(h, Part1);
        assert!(matches!(h.score, Score::Four));

        let h = "54455 999";
        let h = parse_hand(h, Part1);
        assert!(matches!(h.score, Score::Full));

        let h = "54655 999";
        let h = parse_hand(h, Part1);
        assert!(matches!(h.score, Score::Three));

        let h = "5445K 999";
        let h = parse_hand(h, Part1);
        assert!(matches!(h.score, Score::TwoPair));

        let h = "544AK 999";
        let h = parse_hand(h, Part1);
        assert!(matches!(h.score, Score::Pair));

        let h = "534AK 999";
        let h = parse_hand(h, Part1);
        assert!(matches!(h.score, Score::High));

        let h = "24443 999";
        let h = parse_hand(h, Part1);
        assert!(matches!(h.score, Score::Three));

        let h = "KK677 999";
        let h = parse_hand(h, Part1);
        assert!(matches!(h.score, Score::TwoPair));
    }

    #[test]
    fn test_score_part2() {
        let h = "32T3K 999";
        let h = parse_hand(h, Part2);
        assert!(matches!(h.score, Score::Pair));

        let h = "KK677 999";
        let h = parse_hand(h, Part2);
        assert!(matches!(h.score, Score::TwoPair));

        let h = "T55J5 999";
        let h = parse_hand(h, Part2);
        assert!(matches!(h.score, Score::Four));

        let h = "KTJJT 999";
        let h = parse_hand(h, Part2);
        assert!(matches!(h.score, Score::Four));

        let h = "QQQJA 999";
        let h = parse_hand(h, Part2);
        assert!(matches!(h.score, Score::Four));
    }
    #[test]
    fn test_order() {
        let h1 = "55555 999";
        let h1 = parse_hand(h1, Part1);
        let h2 = "55554 999";
        let h2 = parse_hand(h2, Part1);

        assert!(h1 > h2);

        let h1 = "33332 999";
        let h1 = parse_hand(h1, Part1);
        let h2 = "2AAAA 999";
        let h2 = parse_hand(h2, Part1);

        assert!(h1 > h2);

        let h1 = "23456 999";
        let h1 = parse_hand(h1, Part1);
        let h2 = "65432 999";
        let h2 = parse_hand(h2, Part1);

        assert!(h1 == h2);

        let h1 = "3A3A3 999";
        let h1 = parse_hand(h1, Part1);
        let h2 = "AA432 999";
        let h2 = parse_hand(h2, Part1);

        assert!(h1 > h2);
    }

    #[test]
    fn test_order_part2() {
        let input = "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";
        let mut hands: Vec<Hand<Part2>> = input.lines().map(|s| parse_hand(s, Part2)).collect();
        hands.sort();
        println!("sorted:");
        for h in hands.iter() {
            println!("{}", std::str::from_utf8(&h.cards).unwrap());
        }
    }

    #[test]
    fn test_part1() {
        println!("{}", Part1.run(Part1.input_data().unwrap()));
    }

    #[test]
    fn test_part2() {
        println!("{}", Part2.run(Part2.input_data().unwrap()));
    }
}
