#![feature(anonymous_lifetime_in_impl_trait)]
#![feature(test)]
extern crate test;

use common_lib::{get_input_cached, IntoRowIter, Result};

const DAY: usize = 13;

fn does_reflect(iter: impl Iterator<Item = Vec<char>>, pos: usize, errors: usize) -> bool {
    errors
        == iter
            .enumerate()
            .map(|(_ln, l)| {
                let forward = l.iter().skip(pos);
                let backward = l[..pos].iter().rev();

                forward
                    .zip(backward)
                    .map(|(a, b)| if a == b { 0 } else { 1 })
                    .sum::<usize>()
            })
            .sum()
}

fn get_refs(game: &Vec<Vec<char>>, errors: usize) -> (Option<usize>, Option<usize>) {
    let line_length = game[0].len();
    let mirror_posx =
        (1..(line_length)).find(|&pos| does_reflect(game.iter().cloned(), pos, errors));

    let row_length = game.len();
    let mirror_posy = (1..(row_length)).find(|&pos| does_reflect(game.row_iter(), pos, errors));
    (mirror_posx, mirror_posy)
}

fn part1(input: &str) -> Result<usize> {
    let mut lines = input.lines();

    let mut total = 0;
    loop {
        let game: Vec<Vec<char>> = lines
            .by_ref()
            .take_while(|l| {
                //dbg!(&l);
                !l.is_empty()
            })
            .map(|l| l.chars().collect())
            .collect();

        if game.is_empty() {
            break;
        }

        let (x, y) = get_refs(&game, 0);

        //println!("{x:?} {y:?}");
        // if let Some(y) = y {
        //     println!("");
        //     for (n, line) in game.iter().enumerate() {
        //         if y == n {
        //             println!("");
        //         }
        //         println!(" {}", line.iter().collect::<String>())
        //     }
        //     println!("");
        // }

        // if let Some(x) = x {
        //     println!("");
        //     for line in game.iter() {
        //         println!(
        //             " {}|{}",
        //             line.iter().take(x).collect::<String>(),
        //             line.iter().skip(x).collect::<String>()
        //         )
        //     }
        //     println!("");
        // }

        // if x.is_none() && y.is_none() {
        //     println!("");
        //     for line in game.iter() {
        //         println!(" {}", line.iter().collect::<String>())
        //     }
        //     println!("");
        // }

        let val = x.unwrap_or(0) + y.unwrap_or(0) * 100;
        total += val;
    }

    Ok(total)
}

fn part2(input: &str) -> Result<usize> {
    let mut lines = input.lines();

    let mut total = 0;
    loop {
        let game: Vec<Vec<char>> = lines
            .by_ref()
            .take_while(|l| {
                //dbg!(&l);
                !l.is_empty()
            })
            .map(|l| l.chars().collect())
            .collect();

        if game.is_empty() {
            break;
        }

        let (x, y) = get_refs(&game, 1);

        let val = x.unwrap_or(0) + y.unwrap_or(0) * 100;
        total += val;
    }

    Ok(total)
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
