#![feature(test)]
extern crate test;

use common_lib::{get_input_cached, Result};
use geo::Contains;
use geo::Coord;
use geo::LineString;
use geo::Polygon;
use pathfinding::directed::dijkstra::{build_path, dijkstra_all};
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

const DAY: usize = 10;
const SIZE_X: usize = 140;
const SIZE_Y: usize = 140;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Pipe {
    x: usize,
    y: usize,
    map: Rc<Vec<Vec<char>>>,
}

impl fmt::Debug for Pipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = self.map.get(self.x).and_then(|l| l.get(self.y)).copied();
        write!(f, "({}, {}) {symbol:?}", self.x, self.y)
    }
}

impl Pipe {
    fn neighbour(&self, x_offset: isize, y_offset: isize) -> (Pipe, usize) {
        (
            Pipe {
                x: (self.x as isize + x_offset) as usize,
                y: (self.y as isize + y_offset) as usize,
                map: self.map.clone(),
            },
            1,
        )
    }

    fn north(&self) -> (Pipe, usize) {
        self.neighbour(-1, 0)
    }

    fn east(&self) -> (Pipe, usize) {
        self.neighbour(0, 1)
    }

    fn south(&self) -> (Pipe, usize) {
        self.neighbour(1, 0)
    }

    fn west(&self) -> (Pipe, usize) {
        self.neighbour(0, -1)
    }

    fn get_neighbours(&self) -> Vec<(Self, usize)> {
        let symbol = self.map[self.x][self.y];

        match symbol {
            '|' | 'S' => vec![self.north(), self.south()], // Hardcode start symbol because we are dirty
            '-' => vec![self.east(), self.west()],
            'L' => vec![self.north(), self.east()],
            'J' => vec![self.north(), self.west()],
            '7' => vec![self.south(), self.west()],
            'F' => vec![self.south(), self.east()],
            '.' => panic!(". is not a Pipe Segment"),
            'S' => panic!("don't know what to do"),
            s => panic!("invalid symbol {s}"),
        }
    }
}

fn part1(parents: &HashMap<Pipe, (Pipe, usize)>) -> Result<usize> {
    let max_cost = parents
        .values()
        .map(|(_, cost)| cost)
        .copied()
        .max()
        .unwrap();
    Ok(max_cost)
}

fn part2(parents: &mut HashMap<Pipe, (Pipe, usize)>) -> Result<usize> {
    let (target, max_cost) = parents
        .iter()
        .max_by_key(|(_, (_, cost))| cost)
        .map(|(target, (_, max_cost))| (target.clone(), *max_cost))
        .unwrap();

    let path1 = build_path(&target, parents);

    let last_node = parents
        .iter()
        .find(|(pipe, (_, cost))| *cost == max_cost - 1 && *pipe != path1.last().unwrap())
        .unwrap()
        .0
        .clone();

    if let Some(parent) = parents.get_mut(&target) {
        *parent = (last_node, max_cost - 1);
    }

    let path2 = build_path(&target, parents);
    let points = path1
        .iter()
        .chain(path2.iter().rev().skip(1))
        .map(|p| Coord {
            x: p.x as f64,
            y: p.y as f64,
        });
    let polygon = Polygon::new(LineString::from_iter(points), vec![]);

    let mut points_inside = 0;
    for x in 0..SIZE_X {
        for y in 0..SIZE_Y {
            if polygon.contains(&Coord {
                x: x as f64,
                y: y as f64,
            }) {
                points_inside += 1;
            }
        }
    }

    Ok(points_inside)
}

fn main() -> Result<()> {
    let input = get_input_cached(DAY, false)?;
    let map: Rc<Vec<Vec<_>>> = Rc::new(input.lines().map(|l| l.chars().collect()).collect());
    let start = Pipe { x: 25, y: 77, map }; // Hardcode start pos because we are dirty
                                            //let start = Pipe { x: 1, y: 1, map }; // Hardcode start pos because we are dirty

    let mut parents = dijkstra_all(&start, Pipe::get_neighbours);

    print!("Part One: ");
    println!("{}", part1(&parents)?);
    print!("Part Two: ");
    println!("{}", part2(&mut parents)?);

    Ok(())
}
