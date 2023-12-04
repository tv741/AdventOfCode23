use common_lib::get_input_cached;
use std::collections::{HashSet, VecDeque};

#[derive(Clone, Debug)]
struct Card {
    no: u64,
    winning: HashSet<u64>,
    own: HashSet<u64>,
    copies: u64,
}

impl Card {
    fn from_str(s: &str) -> Self {
        let mut parts = s.split(|c| c == ':' || c == '|');
        let no = parts.next().unwrap().split_whitespace().nth(1).unwrap().parse::<u64>().unwrap();
        let winning = parts.next().unwrap().split_whitespace().map(|s|s.parse::<u64>().unwrap()).collect();
        let own = parts.next().unwrap().split_whitespace().map(|s|s.parse::<u64>().unwrap()).collect();

        Self {no, winning, own, copies: 1}
    }

    fn wins(&self) -> u64 {
        self.winning.intersection(&self.own).count() as u64
    } 
}

fn main() {
    let input = get_input_cached(4, false);
    let mut sum1 = 0;

    for line in input.lines() {
        let card = Card::from_str(line);

        let wins = card.wins();
        let points = if wins == 0 {0} else { 2u64.pow(wins as u32 -1) };
        sum1 += points;
        println!("Game {}: {wins} wins, {points} points", card.no);
    }
    println!("Part One: {sum1}");

    let mut cards: VecDeque<Card> = input.lines().map(|l| Card::from_str(l)).collect();
    let mut processed = Vec::new();

    while !cards.is_empty() {
        let card = cards.pop_front().unwrap();
        let wins = card.wins();
        
        cards.iter_mut().take(wins as usize).for_each(|c| c.copies += card.copies);
        processed.push(card);
    }
    let sum2: u64 = processed.iter().map(|c|c.copies).sum();
    println!("Part Two: {}", sum2);

}
