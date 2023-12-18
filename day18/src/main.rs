#![feature(test)]
extern crate test;

use common_lib::{get_input_cached, IntoRowIter, Point, Result};

const DAY: usize = 18;

fn fill(map: &mut [Vec<u32>], pos: Point, size_x: usize, size_y: usize) {
    let mut stack = Vec::<Point>::new();
    stack.push(pos);

    while let Some(p) = stack.pop() {
        if map[p.x][p.y] != 0 {
            continue;
        } else {
            map[p.x][p.y] = 42;

            for dir in [(0, 1), (-1, 0), (0, -1), (1, 0)] {
                let next_pos = p + dir;
                //dbg!(next_pos);
                if next_pos.x < size_x && next_pos.y < size_y {
                    stack.push(next_pos);
                }
            }
        }
    }
}

fn part1(input: &str) -> Result<usize> {
    let steps_iter = input.lines().map(|l| l.split_whitespace());

    let mut min_x: isize = 0;
    let mut max_x: isize = 0;
    let mut min_y: isize = 0;
    let mut max_y: isize = 0;
    let mut steps = Vec::<((isize, isize), usize)>::new();

    let mut pos = (0, 0);
    for mut step in steps_iter {
        let dir_str = step.next().unwrap();
        let len = step.next().unwrap().parse::<usize>().unwrap();

        let dir = match dir_str {
            "R" => (0, 1),
            "L" => (0, -1),
            "D" => (1, 0),
            "U" => (-1, 0),
            _ => panic!(),
        };

        steps.push((dir, len));

        for _ in 0..len {
            pos.0 += dir.0;
            pos.1 += dir.1;
        }
        min_x = min_x.min(pos.0);
        max_x = max_x.max(pos.0);
        max_y = max_y.max(pos.1);
        min_y = min_y.min(pos.1);
    }

    dbg!((min_x, max_x, min_y, max_y,));

    let size_x = (max_x - min_x + 3) as usize;
    let size_y = (max_y - min_y + 3) as usize;

    dbg!((size_x, size_y));

    let mut map = vec![vec![0u32; size_y]; size_x];

    let mut pos = Point {
        x: (-min_x + 1) as usize,
        y: (-min_y + 1) as usize,
    };
    dbg!(pos);

    map[pos.x][pos.y] = 0xffffff;

    for (dir, len) in steps {
        for _ in 0..len {
            pos += dir;
            map[pos.x][pos.y] = 0xffffff;
        }
    }

    // for x in 0..100 {
    //     for y in 0..100 {
    //         if map[x][y] != 0 {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!(" ");
    // }
    // println!(" ");

    fill(&mut map, Point { x: 0, y: 0 }, size_x, size_y);

    let mut total = 0;
    for x in 0..size_x {
        for y in 0..size_y {
            if map[x][y] == 42 {
                //print!("*");
            } else if map[x][y] > 42 {
                //print!("#");
                total += 1;
            } else {
                //print!(".");
                total += 1;
            }
        }
        //println!(" ");
    }
    //println!(" ");

    Ok(total)
}

fn part2(input: &str) -> Result<usize> {
    let steps_iter = input.lines().map(|l| l.split_whitespace());

    let mut min_x: isize = 0;
    let mut max_x: isize = 0;
    let mut min_y: isize = 0;
    let mut max_y: isize = 0;
    let mut steps = Vec::<((isize, isize), usize)>::new();

    let mut pos = (0, 0);
    for step in steps_iter {
        let hex = step.skip(2).next().unwrap();
        let len = usize::from_str_radix(&hex[2..7], 16).unwrap();
        let dir_str = &hex[7..8];

        let dir = match dir_str {
            "0" => (0, 1),
            "2" => (0, -1),
            "1" => (1, 0),
            "3" => (-1, 0),
            _ => panic!(),
        };

        steps.push((dir, len));

        for _ in 0..len {
            pos.0 += dir.0;
            pos.1 += dir.1;
        }
        min_x = min_x.min(pos.0);
        max_x = max_x.max(pos.0);
        max_y = max_y.max(pos.1);
        min_y = min_y.min(pos.1);
    }

    dbg!((min_x, max_x, min_y, max_y,));

    let size_x = (max_x - min_x + 3) as usize;
    let size_y = (max_y - min_y + 3) as usize;

    dbg!((size_x, size_y));

    // /let mut map = vec![vec![0u32; size_y]; size_x];

    let mut pos = Point {
        x: (-min_x + 1) as usize,
        y: (-min_y + 1) as usize,
    };
    dbg!(pos);

    // map[pos.x][pos.y] = 0xffffff;

    // for (dir, len) in steps {
    //     for _ in 0..len {
    //         pos += dir;
    //         map[pos.x][pos.y] = 0xffffff;
    //     }
    // }

    // for x in 0..100 {
    //     for y in 0..100 {
    //         if map[x][y] != 0 {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!(" ");
    // }
    // println!(" ");

    // fill(&mut map, Point { x: 0, y: 0 }, size_x, size_y);

    // let mut total = 0;
    // for x in 0..size_x {
    //     for y in 0..size_y {
    //         if map[x][y] == 42 {
    //             //print!("*");
    //         } else if map[x][y] > 42 {
    //             //print!("#");
    //             total += 1;
    //         } else {
    //             //print!(".");
    //             total += 1;
    //         }
    //     }
    //     //println!(" ");
    // }
    // //println!(" ");

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
