#![feature(test)]
extern crate test;

use std::collections::VecDeque;

use common_lib::{get_input_cached, ParseNums, Result};

const DAY: usize = 9;

struct Diffs<I>
where
    I: Iterator,
{
    parent: I,
    prev: Option<I::Item>,
}

impl<I> Iterator for Diffs<I>
where
    I: Iterator,
    I::Item: Copy + std::ops::Sub<I::Item, Output = I::Item>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.parent.next() {
            if let Some(prev) = self.prev {
                self.prev = Some(next);
                Some(next - prev)
            } else {
                let second_next = self.parent.next().unwrap();
                self.prev = Some(second_next);
                Some(second_next - next)
            }
        } else {
            None
        }
    }
}

trait IntoDiffs<I>
where
    I: Iterator,
    I::Item: Copy + std::ops::Sub<I::Item, Output = I::Item>,
{
    fn diffs(self) -> Diffs<I>;
}

impl<I> IntoDiffs<I> for I
where
    I: Iterator,
    I::Item: Copy + std::ops::Sub<I::Item, Output = I::Item>,
{
    fn diffs(self) -> Diffs<I> {
        Diffs {
            parent: self,
            prev: None,
        }
    }
}

fn part1(input: &str) -> Result<isize> {
    let mut estimates = Vec::<isize>::new();

    for seq in input
        .lines()
        .map(|s| -> Vec<_> { s.parse_nums().collect() })
    {
        let mut new_seq: Vec<_>;
        let mut seqs = vec![seq];
        loop {
            let prev_seq = seqs.last().unwrap();
            if prev_seq.iter().all(|&n| n == 0) {
                break;
            }
            new_seq = prev_seq.iter().copied().diffs().collect();
            seqs.push(new_seq);
        }

        let mut prev_last_item = 0;
        for seq in seqs.iter_mut().rev().skip(1) {
            prev_last_item += seq.last().unwrap();
            seq.push(prev_last_item);
        }
        estimates.push(prev_last_item);
    }

    Ok(estimates.iter().sum())
}

fn part2(input: &str) -> Result<isize> {
    let mut estimates = Vec::<isize>::new();

    for seq in input
        .lines()
        .map(|s| -> VecDeque<_> { s.parse_nums().collect() })
    {
        let mut new_seq;
        let mut seqs = vec![seq];
        loop {
            let prev_seq = seqs.last().unwrap();
            if prev_seq.iter().all(|&n| n == 0) {
                break;
            }
            new_seq = prev_seq.iter().copied().diffs().collect();
            seqs.push(new_seq);
        }

        let mut prev_first_item = 0;
        for seq in seqs.iter_mut().rev().skip(1) {
            prev_first_item = seq.front().unwrap() - prev_first_item;
            seq.push_front(prev_first_item);
        }
        estimates.push(prev_first_item);
    }

    Ok(estimates.iter().sum())
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
