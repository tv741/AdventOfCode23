#![feature(test)]
extern crate test;

use common_lib::{get_input_cached, IntoRowIter, Result};
use std::collections::HashMap;

const DAY: usize = 14;

fn sort(array: &mut Vec<char>) {
    for i in 0..array.len() {
        for j in 0..array.len() - i - 1 {
            match (array[j + 1], array[j]) {
                ('.', 'O') => {}
                ('O', '.') => array.swap(j, j + 1),
                _ => {}
            }
        }
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut lines = input.lines();

    let game: Vec<Vec<char>> = lines
        .by_ref()
        .take_while(|l| {
            //dbg!(&l);
            !l.is_empty()
        })
        .map(|l| l.chars().collect())
        .collect();

    let mut game_transposed: Vec<Vec<char>> = game.row_iter().collect();

    let mut sum = 0;
    for line in game_transposed.iter() {
        let mut next = 0;
        for (n, c) in line.iter().enumerate() {
            match c {
                'O' => {
                    sum += line.len() - next;
                    next += 1;
                }
                '#' => {
                    next = n + 1;
                }
                _ => {}
            }
        }
    }
    dbg!(sum);

    // println!("");
    // game.iter()
    //     .for_each(|l| println!("{}", l.iter().collect::<String>()));
    // println!("");

    game_transposed.iter_mut().for_each(|l| sort(l));

    let game: Vec<Vec<char>> = game_transposed.row_iter().collect();

    // println!("");
    // game.iter()
    //     .for_each(|l| println!("{}", l.iter().collect::<String>()));
    // println!("");

    let lines = game.len();
    let sum = game
        .iter()
        .map(|l| l.iter().filter(|&&c| c == 'O').count())
        .enumerate()
        .fold(0, |acc, (n, count)| acc + count * (lines - n));

    Ok(sum)
}

fn rot(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = vec![vec!['.'; map.len()]; map[0].len()];
    for col in 0..map[0].len() {
        for row in 0..map.len() {
            result[col][map[0].len() - 1 - row] = map[row][col];
        }
    }
    result
}

fn part2(input: &str) -> Result<usize> {
    let mut lines = input.lines();

    let mut game: Vec<Vec<char>> = lines
        .by_ref()
        .take_while(|l| {
            //dbg!(&l);
            !l.is_empty()
        })
        .map(|l| l.chars().collect())
        .collect();

    let mut game_transposed: Vec<Vec<char>>;

    let mut results: HashMap<Vec<Vec<char>>, (usize, usize)> = HashMap::new();
    let mut cycle_len = 0;

    for n in 0..1_000_000_000 {
        for _ in 0..4 {
            game_transposed = game.row_iter().collect();
            game_transposed.iter_mut().for_each(|l| sort(l));
            game = game_transposed.row_iter().collect();
            game = rot(game);
        }

        let lines = game.len();
        let sum = game
            .iter()
            .map(|l| l.iter().filter(|&&c| c == 'O').count())
            .enumerate()
            .fold(0, |acc, (n, count)| acc + count * (lines - n));

        //println!("{n} {}", sum);

        if results.contains_key(&game) {
            cycle_len = n - results.get(&game).unwrap().0;
            break;
        } else {
            results.insert(game.clone(), (n, sum));
        }

        // println!("");
        // game.iter()
        //     .for_each(|l| println!("{}", l.iter().collect::<String>()));
        // println!("");
    }

    let rest = (1_000_000_000 - results.len() - 1) % cycle_len;
    println!(
        "Setup: {}, Cycle length: {}, Rest: {}",
        results.len(),
        cycle_len,
        rest
    );

    for _ in 0..rest {
        for _ in 0..4 {
            game_transposed = game.row_iter().collect();
            game_transposed.iter_mut().for_each(|l| sort(l));
            game = game_transposed.row_iter().collect();
            game = rot(game);
        }
    }

    let lines = game.len();
    let sum = game
        .iter()
        .map(|l| l.iter().filter(|&&c| c == 'O').count())
        .enumerate()
        .fold(0, |acc, (n, count)| acc + count * (lines - n));

    Ok(sum)
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
