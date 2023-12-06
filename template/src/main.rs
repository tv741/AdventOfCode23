#![feature(test)]
extern crate test;

use common_lib::get_input_cached;

const DAY: usize = 0;

fn part1(input: &str) -> usize {
    0
}

fn part2(input: &str) -> usize {
    0
}

fn main() {
    let input = get_input_cached(DAY, false);

    println!("Part One: {}", part1(&input));
    println!("Part Two: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = get_input_cached(DAY, false);
            b.iter(|| black_box(part1(&input)));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = get_input_cached(DAY, false);
        b.iter(|| black_box(part2(&input)));
    }
}