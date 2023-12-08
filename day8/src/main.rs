#![feature(test)]
extern crate test;

use std::collections::HashMap;

use common_lib::{get_input_cached, Result};

const DAY: usize = 8;

fn parse(input: &str) -> Result<(&str, HashMap::<&str, (&str, &str)>)> {
    let mut lines = input.lines();
    
    let directions = lines.next().unwrap();
    dbg!(directions);

    let map = HashMap::<&str, (&str, &str)>::from_iter(lines.skip(1).map(|l|{
        let parts: Vec<_> = l.split(|c| " =(,)".contains(c)).filter(|p| !p.is_empty()).collect();
        (parts[0], (parts[1], parts[2]))
    }));

    Ok((directions, map))
}

fn part1(input: &str) -> Result<usize> {
    let (directions, map) = parse(input)?;

    let dst = "ZZZ";
    let mut curr = "AAA";
    let mut dirs = directions.chars();
    let mut steps = 0;

    while curr != dst {
        let dir = dirs.next().unwrap_or_else(||{
            dirs = directions.chars();
            dirs.next().unwrap()
        });
        let (left, right) = map.get(curr).unwrap();
        curr = if dir == 'L' {
            left
        } else {
            right
        };
        steps += 1;
    }

    Ok(steps)
}

fn part2(input: &str) -> Result<usize> {
    let (directions, map) = parse(input)?;

    let mut curr: Vec<_> = map.keys().filter(|s| s.ends_with('A')).map(|&r|r).collect();
    
    dbg!(&curr);
    
    let mut zs_visited = HashMap::new();
    let mut cycles = HashMap::new();
    let mut steps = 0;
    
    while cycles.len() < curr.len() {
        for dir in directions.chars() {
            for c in curr.iter_mut() {
                let (left, right) = map.get(c).unwrap();
                *c = if dir == 'L' {
                    left
                } else {
                    right
                };
    
                if c.ends_with('Z') {
                    if !zs_visited.contains_key(c) {
                        println!("step {steps}: found {}", *c);
                        zs_visited.insert(*c, steps);
                    } else if !cycles.contains_key(c) {
                        cycles.insert(*c,steps - zs_visited[c]);
                        println!("step {steps}: found cycle {}({})", *c, steps - zs_visited[c]);
                    }
                }
            }
    
            steps += 1;
        }
    }

    Ok(cycles.values().copied().reduce(num::integer::lcm).unwrap())

}

fn main() -> Result<()> {
    let input = get_input_cached(DAY, false)?;

    //println!("Part One: {}", part1(&input)?);
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
