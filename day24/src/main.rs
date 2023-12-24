#![feature(test)]
extern crate test;

use common_lib::{get_input_cached, Result};
use geo::{
    geometry::{Coord, Line},
    Intersects,
};
use itertools::Itertools;

const DAY: usize = 24;

fn part1(input: &str) -> Result<usize> {
    let storms: Vec<_> = input
        .lines()
        .map(|l| {
            let mut iter = l
                .split(|c: char| !c.is_ascii_digit() && c != '-')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<isize>().unwrap());

            let x = iter.next().unwrap() as f64;
            let y = iter.next().unwrap() as f64;
            let dx = iter.nth(1).unwrap() as f64;
            let dy = iter.next().unwrap() as f64;

            (x, y, dx, dy)
        })
        .collect();

    //let min = 7.;
    let min = 200000000000000.;

    //let max = 27.;
    let max = 400000000000000.;

    let intersections = storms
        .iter()
        .tuple_combinations::<(_, _)>()
        .filter(|(l, m)| {
            let (ax, ay, bx, by) = l;
            let (cx, cy, dx, dy) = m;
            //print!("({ax}, {ay}) x ({cx}, {cy})");
            let denom = dx * by - dy * bx;
            if denom != 0.0 {
                let u = (bx * (cy - ay) + by * (ax - cx)) / denom;
                let t = (dx * (ay - cy) + dy * (cx - ax)) / (bx * dy - by * dx);
                let ix = cx + u * dx;
                let iy = cy + u * dy;
                //print!(" = ({u:.3}, {t:.3}) ({ix:.3}, {iy:.3})");
                if u < 0. || t < 0. {
                    //println!(" -> in the past.");
                    false
                } else if min < ix && ix <= max && min < iy && iy <= max {
                    //println!(" -> inside!");
                    true
                } else {
                    //println!(" -> outside.");
                    false
                }
            } else {
                //println!(" = None");
                false
            }
        })
        .count();

    Ok(intersections)
}

fn part2(input: &str) -> Result<usize> {
    Ok(0)
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
