#![feature(test)]
extern crate test;

use common_lib::{get_input_cached, Result};

const DAY: usize = 0;

fn part1(input: &str) -> Result<usize> {
    Ok(0)
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
