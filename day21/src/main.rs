#![feature(test)]
extern crate test;

use common_lib::{get_input_cached, IPoint, Point, Result};
use std::collections::HashSet;
use std::mem::swap;

const DAY: usize = 21;

fn get_neighbours(map: &'_ [Vec<char>], pos: Point) -> impl Iterator<Item = Point> + '_ {
    let offsets = vec![(0, 1), (-1, 0), (0, -1), (1, 0)];
    let size = Point {
        x: map.len(),
        y: map[0].len(),
    };

    offsets.into_iter().filter_map(move |(dx, dy)| {
        let (x, y) = (pos.x as isize, pos.y as isize);

        if x + dx >= 0 && x + dx < size.x as isize && y + dy >= 0 && y + dy < size.y as isize {
            let next = Point {
                x: (x + dx) as usize,
                y: (y + dy) as usize,
            };

            if map[next.x][next.y] == '.' || map[next.x][next.y] == 'S' {
                return Some(next);
            }
        }
        None
    })
}

fn part1(input: &str) -> Result<usize> {
    let mut start = Point { x: 0, y: 0 };
    let map: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(x, l)| {
            l.chars()
                .enumerate()
                .map(|(y, c)| {
                    if c == 'S' {
                        start = Point { x, y };
                    }
                    c
                })
                .collect()
        })
        .collect();

    let mut reachable_plots = HashSet::new();
    reachable_plots.insert(start);

    let mut new_reachable_plots = HashSet::new();

    for _ in 0..64 {
        reachable_plots
            .drain()
            .for_each(|p| new_reachable_plots.extend(get_neighbours(&map, p)));
        swap(&mut reachable_plots, &mut new_reachable_plots);

        print!("{}[2J", 27 as char);
        for (x, line) in map.iter().enumerate() {
            for (y, plot) in line.iter().enumerate() {
                if reachable_plots.contains(&Point { x, y }) {
                    print!("O");
                } else {
                    print!("{}", plot);
                }
            }
            println!(" ");
        }
        //println!("{} plots reachable.\n", reachable_plots.len());
    }

    Ok(reachable_plots.len())
}

fn get_neighbours_unlimited(
    map: &'_ [Vec<char>],
    pos: IPoint,
) -> impl Iterator<Item = IPoint> + '_ {
    let offsets = vec![(0, 1), (-1, 0), (0, -1), (1, 0)];
    let size = IPoint {
        x: map.len() as isize,
        y: map[0].len() as isize,
    };

    offsets.into_iter().filter_map(move |(dx, dy)| {
        let (x, y) = (pos.x, pos.y);

        let next = IPoint {
            x: x + dx,
            y: y + dy,
        };

        let x = (next.x.rem_euclid(size.x)) as usize;
        let y = (next.y.rem_euclid(size.y)) as usize;
        if map[x][y] == '.' || map[x][y] == 'S' {
            Some(next)
        } else {
            None
        }
    })
}

fn get_next_reachable(map: &[Vec<char>], mut reachable: HashSet<IPoint>) -> HashSet<IPoint> {
    let mut new_reachable_plots = HashSet::new();
    reachable
        .drain()
        .for_each(|p| new_reachable_plots.extend(get_neighbours_unlimited(map, p)));

    new_reachable_plots
}

fn part2(input: &str) -> Result<usize> {
    let mut start = IPoint { x: 0, y: 0 };
    let map: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(x, l)| {
            l.chars()
                .enumerate()
                .map(|(y, c)| {
                    if c == 'S' {
                        start = IPoint {
                            x: x as isize,
                            y: y as isize,
                        };
                    }
                    c
                })
                .collect()
        })
        .collect();

    let size = Point {
        x: map.len(),
        y: map[0].len(),
    };
    dbg!(size);
    //let steps = 26501365;
    let steps = 7;
    let setup = size.x / 2;
    let chunks = (steps - setup) / size.x;
    let a = chunks * 2 + 1;
    let rest = (steps - setup) % size.x;

    dbg!((a, setup, chunks, rest));

    let reachable_full_chunk: usize = map
        .iter()
        .map(|l| l.iter().filter(|c| ['.', 'S'].contains(c)).count())
        .sum();

    let full_chunks = a.pow(2);
    let total = full_chunks * reachable_full_chunk / 2;
    println!("{full_chunks} * {reachable_full_chunk} = {total}");

    let mut reachable_plots = HashSet::new();
    reachable_plots.insert(start);

    for _ in 0..7 {
        reachable_plots = get_next_reachable(&map, reachable_plots);

        for x in 0..16 {
            for y in 0..16 {
                if reachable_plots.contains(&IPoint {
                    x: (x - 6),
                    y: (y - 6),
                }) {
                    print!("O");
                } else {
                    print!(".");
                }
            }
            println!(" ");
        }
        println!(" ");
    }

    dbg!(reachable_plots.len());

    Ok(total)
}

fn main() -> Result<()> {
    let input = get_input_cached(DAY, true)?;

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
