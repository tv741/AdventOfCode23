#![feature(iter_intersperse)]
#![feature(let_chains)]
#![feature(test)]
extern crate test;

use common_lib::{get_input_cached, ParseNums, Result};
use rayon::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;

const DAY: usize = 12;

fn is_valid(s: &str, groups: &[usize]) -> bool {
    let found: Vec<usize> = s
        .split('.')
        .filter(|s| !s.is_empty())
        .map(|s| s.len())
        .collect();
    found == groups
}

fn get_permutation(s: &str, i: usize) -> String {
    let mut res: Vec<_> = s.chars().collect();

    for (n, c) in res.iter_mut().filter(|c| **c == '?').enumerate() {
        *c = if 1 << n & i != 0 { '#' } else { '.' };
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
            if valid {
                arrangements += 1;
            }
        }
        //dbg!(arrangements);
        total += arrangements;
    }

    Ok(total)
}

type Cache = HashMap<(usize, usize, usize), usize>;

fn solve(
    cache: &mut Cache,
    springs: &[char],
    groups: &[usize],
    curr_len: usize,
    debug: String,
) -> usize {
    // println!(
    //     "{debug} -> {}, {groups:?}, {curr_len:?}",
    //     springs.iter().collect::<String>()
    // );
    let key = (springs.len(), groups.len(), curr_len);
    if let Some(res) = cache.get(&key).copied() {
        res
    } else {
        let res = _solve(cache, springs, groups, curr_len, debug);
        cache.insert(key, res);
        res
    }
}

fn handle_dot(
    cache: &mut Cache,
    springs: &[char],
    groups: &[usize],
    curr_len: usize,
    debug: &str,
) -> usize {
    if curr_len > 0 {
        if let Some(&group) = groups.first()
            && curr_len == group
        {
            solve(
                cache,
                &springs[1..],
                &groups[1..],
                0,
                debug.to_string() + ".",
            )
        } else {
            0
        }
    } else {
        solve(cache, &springs[1..], groups, 0, debug.to_string() + ".")
    }
}

fn handle_spring(
    cache: &mut Cache,
    springs: &[char],
    groups: &[usize],
    curr_len: usize,
    debug: &str,
) -> usize {
    solve(
        cache,
        &springs[1..],
        groups,
        curr_len + 1,
        debug.to_string() + "#",
    )
}

fn _solve(
    cache: &mut Cache,
    springs: &[char],
    groups: &[usize],
    curr_len: usize,
    debug: String,
) -> usize {
    if let Some(c) = springs.first() {
        match c {
            '.' => handle_dot(cache, springs, groups, curr_len, &debug),
            '#' => handle_spring(cache, springs, groups, curr_len, &debug),
            '?' => {
                handle_dot(cache, springs, groups, curr_len, &debug)
                    + handle_spring(cache, springs, groups, curr_len, &debug)
            }
            _ => panic!("Invalid char"),
        }
    } else {
        // empty
        if groups.is_empty() && springs.is_empty() && curr_len == 0
            || groups.len() == 1 && curr_len == *groups.first().unwrap()
        {
            //println!("found one: {debug}");
            1
        } else {
            0
        }
    }
}

fn part2(input: &str) -> Result<usize> {
    let lines: Vec<_> = input.lines().collect();
    let total = lines
        .iter()
        .map(|line| {
            let mut cache: Cache = HashMap::new();
            let mut parts = line.split_whitespace();
            let springs: Vec<_> = parts.next().unwrap().chars().collect();
            let groups: Vec<usize> = parts.next().unwrap().parse_nums(',').collect();

            let springs: Vec<_> = (0..5)
                .map(|_| springs.clone())
                .intersperse(vec!['?'])
                .flatten()
                .collect();
            let groups: Vec<usize> = (0..5).flat_map(|_| groups.iter()).copied().collect();

            let arrangements = solve(&mut cache, &springs, &groups, 0, "".to_string());
            //dbg!(arrangements);
            arrangements
        })
        .sum();

    Ok(total)
}

fn part1vs2(input: &str) -> Result<usize> {
    let mut total = 0;
    for (n, line) in input.lines().enumerate() {
        let mut parts = line.split_whitespace();
        let springs = parts.next().unwrap();
        let groups: Vec<usize> = parts.next().unwrap().parse_nums(',').collect();

        let mut arrangements1 = 0;
        let knowns = springs.chars().filter(|c| *c == '#').count();
        let needed = groups.iter().sum::<usize>() - knowns;
        let unknowns = springs.chars().filter(|c| *c == '?').count() as u32;
        for i in (0..2usize.pow(unknowns)).filter(|i| i.count_ones() == needed as u32) {
            let perm = get_permutation(springs, i);
            let valid = is_valid(&perm, &groups);
            if valid {
                arrangements1 += 1;
            }
        }

        let mut cache: Cache = HashMap::new();
        let springs: Vec<_> = springs.chars().collect();
        let arrangements2 = solve(&mut cache, &springs, &groups, 0, "".to_string());

        if arrangements1 != arrangements2 {
            dbg!((springs, n + 1, arrangements1, arrangements2));
            break;
        }

        total += arrangements1;
    }

    Ok(total)
}

fn main() -> Result<()> {
    let input = get_input_cached(DAY, false)?;

    //println!("Part One: {}", part1vs2(&input)?);
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
