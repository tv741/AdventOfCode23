#![feature(test)]
extern crate test;

use common_lib::{get_input_cached, Result, Point};
use itertools::Itertools;

const DAY: usize = 11;

struct RowIter<T>
{
    data: Vec<Vec<T>>,
    n: usize,
}

impl<T: Copy> Iterator for RowIter<T>
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n < self.data[0].len() {
            let res = self.data.iter().map(|l| l[self.n]).collect();
            self.n+=1;
            Some(res)
        } else { None }
    }
}

fn get_expanse<'a>(iter: impl Iterator<Item=Vec<char>>) -> Vec<usize> {
    iter.enumerate().filter_map(|(n, l)| {
        if !l.contains(&'#') {
            Some(n)
        } else { None }
    }).collect()
}

fn part1(input: &str) -> Result<usize> {

    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut galaxies: Vec<_> = map.iter().enumerate().map(|(x, l)| {
        let galaxies: Vec<_> = l.iter().enumerate().filter(|(_y, &c)| c == '#').map(|(y, _c)| Point{x, y}).collect();
        galaxies
    }).flatten().collect();

    let lines_expanse: Vec<_> = get_expanse(map.iter().cloned());
    let row_expanse: Vec<_> = get_expanse(RowIter{data: map, n: 0});

    galaxies.iter_mut().for_each(|g| {
        let lx = lines_expanse.iter().filter(|&&line| line < g.x).count();
        let ly = row_expanse.iter().filter(|&&line| line < g.y).count();
        *g = Point{x: g.x + lx, y: g.y + ly};
    });

    let sum = galaxies.iter().combinations(2).map(|points|{                
        let distance = points[0].manhatten(points[1]);
        distance}
    ).sum();

    Ok(sum)
}

fn part2(input: &str) -> Result<usize> {
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut galaxies: Vec<_> = map.iter().enumerate().map(|(x, l)| {
        let galaxies: Vec<_> = l.iter().enumerate().filter(|(_y, &c)| c == '#').map(|(y, _c)| Point{x, y}).collect();
        galaxies
    }).flatten().collect();

    let lines_expanse: Vec<_> = get_expanse(map.iter().cloned());
    let row_expanse: Vec<_> = get_expanse(RowIter{data: map, n: 0});

    let f = 1000000 - 1;
    galaxies.iter_mut().for_each(|g| {
        let lx = lines_expanse.iter().filter(|&&line| line < g.x).count();
        let ly = row_expanse.iter().filter(|&&line| line < g.y).count();
        *g = Point{x: g.x + (lx * f), y: g.y + (ly * f)};
    });

    let sum = galaxies.iter().combinations(2).map(|points|{                
        let distance = points[0].manhatten(points[1]);
        distance}
    ).sum();

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
