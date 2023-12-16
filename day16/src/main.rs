#![feature(test)]
extern crate test;

use std::collections::HashMap;

use common_lib::{get_input_cached, Result};

const DAY: usize = 16;

fn part1(input: &str) -> Result<usize> {
    let game: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let size_x = game.len() as isize;
    let size_y = game[0].len() as isize;

    // for x in 0..size_x {
    //     for y in 0..size_y {
    //         print!("{}", game[x as usize][y as usize]);
    //     }
    //     println!("");
    // }

    let mut energized = HashMap::<(isize, isize), Vec<(isize, isize)>>::new();
    let mut beams: Vec<((isize, isize), (isize, isize))> = vec![((0, 0), (0, 1))];

    while let Some(((mut x, mut y), (mut dx, mut dy))) = beams.pop() {
        //println!("new ray:");
        loop {
            if let Some(e) = energized.get_mut(&(x, y)) {
                if e.contains(&(dx, dy)) {
                    break;
                } else {
                    e.push((dx, dy));
                }
            } else {
                energized.insert((x, y), vec![(dx, dy)]);
            }

            let field = game[x as usize][y as usize];
            //print!("{x} {y} {field}");
            match (field, (dx, dy)) {
                ('\\', (1, 0)) => (dx, dy) = (0, 1),
                ('\\', (-1, 0)) => (dx, dy) = (0, -1),
                ('\\', (0, 1)) => (dx, dy) = (1, 0),
                ('\\', (0, -1)) => (dx, dy) = (-1, 0),
                ('/', (1, 0)) => (dx, dy) = (0, -1),
                ('/', (-1, 0)) => (dx, dy) = (0, 1),
                ('/', (0, 1)) => (dx, dy) = (-1, 0),
                ('/', (0, -1)) => (dx, dy) = (1, 0),
                ('|', (0, 1)) | ('|', (0, -1)) => {
                    (dx, dy) = (1, 0);
                    beams.push(((x, y), (-1, 0)))
                }
                ('-', (1, 0)) | ('-', (-1, 0)) => {
                    (dx, dy) = (0, 1);
                    beams.push(((x, y), (0, -1)))
                }
                _ => {}
            }

            //println!(" -> {dx} {dy}");

            let new_x = x + dx;
            let new_y = y + dy;

            if new_x >= 0 && new_x < size_x && new_y >= 0 && new_y < size_y {
                (x, y) = (new_x, new_y);
            } else {
                break;
            }
        }
    }

    // println!("");
    // for x in 0..size_x {
    //     for y in 0..size_y {
    //         if let Some(v) = energized.get(&(x, y)) {
    //             if v.len() == 1 {
    //                 let c = match v[0] {
    //                     (1, 0) => 'v',
    //                     (-1, 0) => '^',
    //                     (0, 1) => '>',
    //                     (0, -1) => '<',
    //                     _ => 'x',
    //                 };
    //                 print!("{c}");
    //             } else {
    //                 print!("{}", v.len());
    //             }
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!("");
    // }

    Ok(energized.len())
}

fn get_energized(game: &[Vec<char>], beam: ((isize, isize), (isize, isize))) -> usize {
    let size_x = game.len() as isize;
    let size_y = game[0].len() as isize;

    let mut energized = HashMap::<(isize, isize), Vec<(isize, isize)>>::new();
    let mut beams: Vec<((isize, isize), (isize, isize))> = vec![beam];

    while let Some(((mut x, mut y), (mut dx, mut dy))) = beams.pop() {
        //println!("new ray:");
        loop {
            if let Some(e) = energized.get_mut(&(x, y)) {
                if e.contains(&(dx, dy)) {
                    break;
                } else {
                    e.push((dx, dy));
                }
            } else {
                energized.insert((x, y), vec![(dx, dy)]);
            }

            let field = game[x as usize][y as usize];
            //print!("{x} {y} {field}");
            match (field, (dx, dy)) {
                ('\\', (1, 0)) => (dx, dy) = (0, 1),
                ('\\', (-1, 0)) => (dx, dy) = (0, -1),
                ('\\', (0, 1)) => (dx, dy) = (1, 0),
                ('\\', (0, -1)) => (dx, dy) = (-1, 0),
                ('/', (1, 0)) => (dx, dy) = (0, -1),
                ('/', (-1, 0)) => (dx, dy) = (0, 1),
                ('/', (0, 1)) => (dx, dy) = (-1, 0),
                ('/', (0, -1)) => (dx, dy) = (1, 0),
                ('|', (0, 1)) | ('|', (0, -1)) => {
                    (dx, dy) = (1, 0);
                    beams.push(((x, y), (-1, 0)))
                }
                ('-', (1, 0)) | ('-', (-1, 0)) => {
                    (dx, dy) = (0, 1);
                    beams.push(((x, y), (0, -1)))
                }
                _ => {}
            }

            let new_x = x + dx;
            let new_y = y + dy;

            if new_x >= 0 && new_x < size_x && new_y >= 0 && new_y < size_y {
                (x, y) = (new_x, new_y);
            } else {
                break;
            }
        }
    }
    energized.len()
}

fn part2(input: &str) -> Result<usize> {
    let game: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let size_x = game.len();
    let size_y = game[0].len();

    let mut max = 0;

    for (x, dx) in [(0, 1), (size_x - 1, -1)] {
        for y in 0..size_y {
            max = max.max(get_energized(&game, ((x as isize, y as isize), (dx, 0))));
        }
    }

    for (y, dy) in [(0, 1), (size_y - 1, -1)] {
        for x in 0..size_x {
            max = max.max(get_energized(&game, ((x as isize, y as isize), (0, dy))));
        }
    }

    Ok(max)
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
