#![feature(iter_intersperse)]
#![feature(test)]
extern crate test;

use common_lib::{get_input_cached, Result, ParseNums};

const DAY: usize = 12;

fn is_valid(s: &str, groups: &[usize]) -> bool {
    let found: Vec<usize> = s.split('.').filter(|s| !s.is_empty()).map(|s| s.len()).collect();
    found == groups
}

fn get_permutation(s: &str, i: usize) -> String {
    let mut res: Vec<_> = s.chars().collect();

    for (n, c) in res.iter_mut().filter(|c| **c == '?').enumerate() {
        *c = if 1 << n & i != 0 {
            '#'
        } else { '.' };
    }

    res.iter().collect()
}

fn part1(input: &str) -> Result<usize> {
    let mut total = 0;
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let springs = parts.next().unwrap();
        let groups: Vec<usize> = parts.next().unwrap().parse_nums(',').collect();

        let mut arrangements = 0;
        let knowns = springs.chars().filter(|c| *c == '#').count();
        let needed = groups.iter().sum::<usize>() - knowns;
        let unknowns = springs.chars().filter(|c| *c == '?').count() as u32;
        for i in (0..2usize.pow(unknowns)).filter(|i| i.count_ones() == needed as u32) {
            let perm = get_permutation(&springs, i);
            let valid = is_valid(&perm, &groups);
            if valid { arrangements += 1; }
        }
        dbg!(arrangements);
        total += arrangements;
    }

    Ok(total)
}

fn part2(input: &str) -> Result<usize> {
    Ok(0)
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
