#![feature(test)]
extern crate test;

use common_lib::{get_input_cached, Result};
use std::{cmp::Ordering, collections::HashMap};

const DAY: usize = 7;
const CARDS: &str = "AKQJT98765432";
const CARDS2: &str = "AKQT98765432J";

fn card_value(c: char) -> usize {
    CARDS.len() - CARDS.find(c).unwrap()
}

fn card_value2(c: char) -> usize {
    CARDS2.len() - CARDS2.find(c).unwrap()
}

fn count_chars(s: &str) -> HashMap<char, usize> {
    let mut chars = HashMap::<char, usize>::new();

    for c in s.chars() {
        if let Some(n) = chars.get_mut(&c) {
            *n += 1;
        } else {
            chars.insert(c, 1);
        }
    }

    chars
}

fn best_two_value(best_two: (usize, usize)) -> usize {
    match best_two {
        (5, _) => 6, // Five of a kind
        (4, _) => 5, // Four of a kind
        (3, 2) => 4, // Full House
        (3, _) => 3, // Tree od a kind
        (2, 2) => 2, // Two pair
        (2, 1) => 1, // One Pair
        _ => 0,      // High Card
    }
}

fn hand_value(hand: &str) -> usize {
    let mut counts: Vec<usize> = count_chars(hand).values().copied().collect();
    counts.sort_by_key(|w| std::cmp::Reverse(*w));

    let best_two = (counts[0], *counts.get(1).unwrap_or(&0));
    best_two_value(best_two)
}

fn hand_value2(hand: &str) -> usize {
    let mut counts = count_chars(hand);
    let jokers = counts.remove(&'J').unwrap_or(0);

    let mut counts: Vec<usize> = counts.values().copied().collect();
    counts.sort_by_key(|w| std::cmp::Reverse(*w));

    let best_two = (
        counts.first().unwrap_or(&0) + jokers,
        *counts.get(1).unwrap_or(&0),
    );
    best_two_value(best_two)
}

fn cmp_hands((a, av): (&str, usize), (b, bv): (&str, usize)) -> Ordering {
    let ord = av.cmp(&bv);
    if ord != Ordering::Equal {
        ord
    } else {
        let ord = a
            .chars()
            .zip(b.chars())
            .map(|(a, b)| card_value(a).cmp(&card_value(b)))
            .find(|&ord| ord != Ordering::Equal)
            .unwrap_or(Ordering::Equal);
        ord
    }
}

fn cmp_hands2((a, av): (&str, usize), (b, bv): (&str, usize)) -> Ordering {
    let ord = av.cmp(&bv);
    if ord != Ordering::Equal {
        ord
    } else {
        let ord = a
            .chars()
            .zip(b.chars())
            .map(|(a, b)| card_value2(a).cmp(&card_value2(b)))
            .find(|&ord| ord != Ordering::Equal)
            .unwrap_or(Ordering::Equal);
        //dbg!((a, b, ord));
        ord
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut hands: Vec<_> = input
        .lines()
        .map(|l| {
            let mut parts = l.split_whitespace();
            let hand = parts.next().unwrap();
            let value = hand_value(hand);
            (hand, value, parts.next().unwrap().parse::<usize>().unwrap())
        })
        .collect();

    hands.sort_by(|(h1, v1, _), (h2, v2, _)| cmp_hands((h1, *v1), (h2, *v2)));

    Ok(hands
        .iter()
        .enumerate()
        .fold(0, |acc, (n, (_hand, _v, bid))| acc + (n + 1) * bid))
}

fn part2(input: &str) -> Result<usize> {
    let mut hands: Vec<_> = input
        .lines()
        .map(|l| {
            let mut parts = l.split_whitespace();
            let hand = parts.next().unwrap();
            let value = hand_value2(hand);
            (hand, value, parts.next().unwrap().parse::<usize>().unwrap())
        })
        .collect();

    hands.sort_by(|(h1, v1, _), (h2, v2, _)| cmp_hands2((h1, *v1), (h2, *v2)));

    Ok(hands
        .iter()
        .enumerate()
        .fold(0, |acc, (n, (_hand, _v, bid))| acc + (n + 1) * bid))
}

fn main() -> Result<()> {
    let input = get_input_cached(DAY, false)?;

    println!("Part One: {}", part1(&input)?);
    println!("Part Two: {}", part2(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = get_input_cached(DAY, false).unwrap();
        b.iter(|| black_box(part1(&input).unwrap()));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = get_input_cached(DAY, false).unwrap();
        b.iter(|| black_box(part2(&input).unwrap()));
    }
}
