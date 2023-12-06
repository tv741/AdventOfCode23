use common_lib::get_input;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::u32, multi::separated_list1, IResult,
};

#[derive(Debug)]
enum Color {
    Blue(u32),
    Red(u32),
    Green(u32),
}

#[derive(Debug)]
struct Hand {
    colors: Vec<Color>,
}

impl Hand {
    fn new(colors: Vec<Color>) -> Self {
        Hand { colors }
    }
}

#[derive(Debug)]
struct Game {
    idx: u32,
    hands: Vec<Hand>,
}

impl Game {
    fn is_possible(&self) -> bool {
        for hand in &self.hands {
            for color in &hand.colors {
                if match color {
                    Color::Red(n) => *n > 12,
                    Color::Green(n) => *n > 13,
                    Color::Blue(n) => *n > 14,
                } {
                    return false;
                };
            }
        }
        true
    }

    fn get_power(&self) -> u64 {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for hand in &self.hands {
            for color in &hand.colors {
                let (n, min) = match color {
                    Color::Red(n) => (n, &mut min_red),
                    Color::Green(n) => (n, &mut min_green),
                    Color::Blue(n) => (n, &mut min_blue),
                };

                *min = u32::max(*n, *min);
            }
        }

        (min_red * min_green * min_blue) as u64
    }
}

fn color_str(input: &str) -> IResult<&str, &str> {
    alt((tag("blue"), tag("red"), tag("green")))(input)
}

fn color(input: &str) -> IResult<&str, Color> {
    let (input, num) = u32(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, color) = color_str(input)?;

    let color = match color {
        "blue" => Color::Blue(num),
        "red" => Color::Red(num),
        "green" => Color::Green(num),
        _ => panic!(),
    };

    Ok((input, color))
}

fn hand(input: &str) -> IResult<&str, Hand> {
    let (input, colors) = separated_list1(tag(", "), color)(input)?;
    Ok((input, Hand::new(colors)))
}

fn hands(input: &str) -> IResult<&str, Vec<Hand>> {
    let (input, hands) = separated_list1(tag("; "), hand)(input)?;
    Ok((input, hands))
}

fn game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, idx) = u32(input)?;

    let (input, _) = tag(": ")(input)?;

    let (input, hands) = hands(input)?;
    let game = Game { idx, hands };

    Ok((input, game))
}

fn main() {
    let input = get_input(2).unwrap();
    println!("{input}");

    let games = input
        .lines()
        .map(|l| game(l).unwrap())
        .map(|(_, g)| g)
        .collect::<Vec<_>>();
    dbg!(games.len());

    let sum1: u32 = games
        .iter()
        .filter(|g| g.is_possible())
        .map(|g| g.idx)
        .sum();
    println!("Part One: {sum1}");

    let sum2: u64 = games.iter().map(|g| g.get_power()).sum();
    println!("Part Two: {sum2}");
}
