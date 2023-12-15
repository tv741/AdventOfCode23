#![feature(ascii_char)]
#![feature(test)]
extern crate test;

use common_lib::{get_input_cached, Result};
use std::fmt::Write;

const DAY: usize = 15;

fn hash(input: &str) -> usize {
    input
        .chars()
        .map(|c| c.as_ascii().unwrap() as usize)
        .fold(0, |acc, c| ((acc + c) * 17) % 256)
}

fn part1(input: &str) -> Result<usize> {
    let sum = input
        .trim()
        .split(',')
        .map(|s| {
            let hash = hash(s);
            //println!("{s} {hash}");
            hash
        })
        .sum();
    Ok(sum)
}

fn part2(input: &str) -> Result<usize> {
    let mut boxes: Vec<Vec<(&str, usize)>> = (0..256).map(|_| Vec::new()).collect();

    for step in input.trim().split(',') {
        //println!("\nAfter \"{step}\"");
        let op = step.chars().find(|&c| c == '=' || c == '-').unwrap();
        let mut iter = step.split(op);
        let label = iter.next().unwrap();
        let label_hash = hash(label);

        let _box = boxes.get_mut(label_hash).unwrap();

        if op == '=' {
            let n = iter.next().unwrap().parse().unwrap();

            if let Some(old_label) = _box.iter_mut().find(|(l, _)| *l == label) {
                *old_label = (label, n);
            } else {
                _box.push((label, n));
            }
        } else if op == '-' {
            if let Some(old_label_idx) = _box.iter().position(|(l, _)| *l == label) {
                _box.remove(old_label_idx);
            }
        } else {
            panic!()
        }

        // boxes.iter().enumerate().for_each(|(n, b)| {
        //     if !b.is_empty() {
        //         println!(
        //             "Box {n}: {}",
        //             b.iter().fold(String::new(), |mut output, (l, n)| {
        //                 let _ = write!(output, "[{l} {n}] ");
        //                 output
        //             })
        //         );
        //     }
        // });
    }

    let mut total = 0;
    for (bn, b) in boxes.iter().enumerate() {
        for (sn, (_l, f)) in b.iter().enumerate() {
            total += (bn + 1) * (sn + 1) * f;
        }
    }

    Ok(total)
}

fn main() -> Result<()> {
    let input = get_input_cached(DAY, true)?;

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
