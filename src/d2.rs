use crate::{read_file, PuzzleRun};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space0},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{delimited, pair},
    Finish, IResult,
};
use std::collections::HashMap;

pub(crate) fn get_runs() -> Vec<Box<dyn PuzzleRun>> {
    vec![Box::new(Part2)]
}

struct Part1;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}

impl std::str::FromStr for Color {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err("invalid color"),
        }
    }
}
#[derive(Debug)]
struct Draw {
    reds: u32,
    greens: u32,
    blues: u32,
}

impl Draw {
    fn new(colors: Vec<(Color, u32)>) -> Self {
        Self {
            reds: colors
                .iter()
                .find(|c| c.0 == Color::Red)
                .map(|c| c.1)
                .unwrap_or(0),
            greens: colors
                .iter()
                .find(|c| c.0 == Color::Green)
                .map(|c| c.1)
                .unwrap_or(0),
            blues: colors
                .iter()
                .find(|c| c.0 == Color::Blue)
                .map(|c| c.1)
                .unwrap_or(0),
        }
    }
}
#[derive(Debug)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
}

impl Game {
    fn new(id: u32, draws: Vec<Draw>) -> Self {
        Self { id, draws }
    }

    fn is_possible(&self, red: u32, green: u32, blue: u32) -> bool {
        self.draws
            .iter()
            .all(|d| d.reds <= red && d.greens <= green && d.blues <= blue)
    }
}
#[derive(Debug)]
struct GameParseError {
    _msg: String,
}

impl From<nom::error::Error<&str>> for GameParseError {
    fn from(value: nom::error::Error<&str>) -> Self {
        GameParseError {
            _msg: value.to_string(),
        }
    }
}
impl std::str::FromStr for Game {
    type Err = GameParseError;

    // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn ws_digit(input: &str) -> IResult<&str, &str> {
            delimited(space0, digit1, space0)(input)
        }
        fn parse_draw_color(input: &str) -> IResult<&str, (Color, u32)> {
            map(
                pair(ws_digit, alt((tag("red"), tag("green"), tag("blue")))),
                |(s1, s2)| (str::parse(s2).unwrap(), str::parse(s1).unwrap()),
            )(input)
        }
        fn parse_draws(input: &str) -> IResult<&str, Draw> {
            map(separated_list1(tag(", "), parse_draw_color), |s| {
                Draw::new(s)
            })(input)
        }
        fn parse_game(input: &str) -> IResult<&str, Game> {
            let (input, _) = tag("Game ")(input)?;
            let (input, id) = map_res(digit1, str::parse)(input)?;
            let (input, _) = tag(": ")(input)?;
            map(separated_list1(tag("; "), parse_draws), move |d| {
                Game::new(id, d)
            })(input)
        }

        match parse_game(s.trim()).finish() {
            Ok(v) => Ok(v.1),
            Err(e) => Err(e.into()),
        }
    }
}
impl PuzzleRun for Part1 {
    fn input_data(&self) -> anyhow::Result<&str> {
        read_file("input/day2.txt")
    }

    fn run(&self, input: &str) -> String {
        fn is_possible(g: &Game) -> bool {
            g.is_possible(12, 13, 14)
        }

        let games: Vec<Game> = input
            .lines()
            .map(str::parse::<Game>)
            .collect::<Result<Vec<Game>, _>>()
            .unwrap();

        let s: u32 = games.iter().filter(|g| is_possible(g)).map(|g| g.id).sum();

        format!("{}", s)
    }
}

struct Part2;

impl PuzzleRun for Part2 {
    fn input_data(&self) -> anyhow::Result<&str> {
        read_file("input/day2.txt")
        /*
        Ok("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green")
        */
    }

    fn run(&self, input: &str) -> String {
        let games: Vec<Game> = input
            .lines()
            .map(str::parse::<Game>)
            .collect::<Result<Vec<Game>, _>>()
            .unwrap();

        let mut sum: u32 = 0;

        for g in games {
            let mut maxes: HashMap<Color, u32> =
                [(Color::Red, 0), (Color::Green, 0), (Color::Blue, 0)].into();
            for d in g.draws {
                let v = maxes.get(&Color::Red).unwrap();
                if d.reds > *v {
                    maxes.insert(Color::Red, d.reds);
                }

                let v = maxes.get(&Color::Green).unwrap();
                if d.greens > *v {
                    maxes.insert(Color::Green, d.greens);
                }

                let v = maxes.get(&Color::Blue).unwrap();
                if d.blues > *v {
                    maxes.insert(Color::Blue, d.blues);
                }
            }
            sum += maxes.get(&Color::Red).unwrap()
                * maxes.get(&Color::Green).unwrap()
                * maxes.get(&Color::Blue).unwrap();
        }
        format!("{}", sum)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_parse() {
        let s = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let g = str::parse::<Game>(s).unwrap();
        assert!(g.id == 1);
        println!("{:?}", g);
    }
}
