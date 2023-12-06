#![feature(test)]
extern crate test;

use common_lib::{get_input_cached, ParseNums, Result};
use itertools::free::join;

fn func(times: Vec<usize>, distances: Vec<usize>) -> usize {
    let mut res1 = 1;
    for (t, d) in times
        .iter()
        .zip(distances.iter())
        .map(|(&t, &d)| (t as f64, d as f64))
    {
        let b = (t.powf(2.0) - 4.0 * d).sqrt();
        let t1 = 0.5 * (t - b);
        let t2 = 0.5 * (b + t);
        let i1 = t1 as isize + 1;
        let i2 = (t2 - 1.0).ceil() as isize;

        let solutions = i2 - i1 + 1;
        res1 *= solutions as usize;
    }

    res1
}

fn part1(input: &str) -> Result<usize> {
    let mut lines = input.lines();
    let times: Vec<_> = lines.next().ok_or("")?.parse_nums().collect();
    let distances: Vec<_> = lines.next().ok_or("")?.parse_nums().collect();

    Ok(func(times, distances))
}

fn part2(input: &str) -> Result<usize> {
    let mut lines2 = input.lines();
    let time = join(lines2.next().ok_or("")?.split_whitespace().skip(1), "").parse::<usize>()?;
    let distance =
        join(lines2.next().ok_or("")?.split_whitespace().skip(1), "").parse::<usize>()?;

    Ok(func(vec![time], vec![distance]))
}

fn main() -> Result<()> {
    let input = get_input_cached(6, false)?;

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
        let input = get_input_cached(6, false).unwrap();
        b.iter(|| black_box(part1(&input)));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = get_input_cached(6, false).unwrap();
        b.iter(|| black_box(part2(&input)));
    }
}
