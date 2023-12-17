#![feature(test)]
extern crate test;

use std::collections::HashMap;

use common_lib::{get_input_cached, Point, Result};

const DAY: usize = 17;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct HPoint {
    pos: Point,
    dir: Option<(isize, isize)>,
    straigt_steps: usize,
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct PPoint {
    parent: Option<HPoint>,
    cost: usize,
    f: usize,
}

struct Map {
    map: Vec<Vec<usize>>,
    size: Point,
    nodes: Vec<(HPoint, PPoint)>,
    closed_list: HashMap<HPoint, PPoint>,
    min_straight: usize,
    max_straight: usize,
}

impl Map {
    fn new(map: Vec<Vec<usize>>, min: usize, max: usize) -> Self {
        let size = Point {
            x: map.len(),
            y: map[0].len(),
        };
        Map {
            map,
            size,
            nodes: Vec::new(),
            closed_list: HashMap::new(),
            min_straight: min,
            max_straight: max,
        }
    }

    fn get_unvisited_neighbours(&mut self, point: &HPoint) -> Vec<(HPoint, usize)> {
        let mut offsets = vec![(0, 1), (-1, 0), (0, -1), (1, 0)];

        // don't go backwards
        if let Some((dx, dy)) = point.dir {
            offsets.retain(|o| *o != (-dx, -dy));

            // don't go straight to long
            if point.straigt_steps >= (self.max_straight - 1) {
                offsets.retain(|o| ((dx, dy) != *o) && (*o != (-dx, -dy)));
            } else if point.straigt_steps < (self.min_straight.saturating_sub(1)) {
                offsets.retain(|o| *o == (dx, dy));
            }
        }

        let neighbours: Vec<(HPoint, usize)> = offsets
            .into_iter()
            .filter_map(|(dx, dy)| {
                let pos = point.pos;
                let (x, y) = (pos.x as isize, pos.y as isize);

                if x + dx >= 0
                    && x + dx < self.size.x as isize
                    && y + dy >= 0
                    && y + dy < self.size.y as isize
                {
                    let next = HPoint {
                        pos: Point {
                            x: (x + dx) as usize,
                            y: (y + dy) as usize,
                        },
                        dir: Some((dx, dy)),
                        straigt_steps: if point.dir == Some((dx, dy)) {
                            point.straigt_steps + 1
                        } else {
                            0
                        },
                    };

                    Some(next)
                } else {
                    None
                }
            })
            .map(|n: HPoint| {
                let cost = self.map[n.pos.x][n.pos.y];
                (n, cost)
            })
            .collect();

        neighbours
    }

    fn a_star(&mut self, start: Point, goal: Point) -> bool {
        let start = HPoint {
            pos: start,
            dir: None,
            straigt_steps: 0,
        };
        let pstart = PPoint {
            parent: None,
            cost: 0,
            f: 0,
        };

        self.nodes.push((start, pstart));

        while let Some((node, pnode)) = self.nodes.pop() {
            self.closed_list.insert(node, pnode.clone());
            if node.pos == goal {
                return true;
            }

            if self.closed_list.len() % 1024 == 0 {
                println!("nodes: {} - {}", self.nodes.len(), self.closed_list.len());
            }

            let neighbours = self.get_unvisited_neighbours(&node);

            for (neighbour, distance) in neighbours {
                if self.closed_list.contains_key(&neighbour) {
                    continue;
                }

                let new_length = pnode.cost + distance;

                if let Some((_h, p)) = self.nodes.iter_mut().find(|(h, _p)| *h == neighbour) {
                    if new_length >= p.cost {
                        continue;
                    }

                    p.parent = Some(node);
                    p.cost = new_length;
                    p.f = new_length + node.pos.manhatten(&goal);
                } else {
                    self.nodes.push((
                        neighbour,
                        PPoint {
                            parent: Some(node),
                            cost: new_length,
                            f: new_length + node.pos.manhatten(&goal),
                        },
                    ));
                }
            }

            self.nodes.sort_by(|(_, a), (_, b)| b.f.cmp(&a.f));
        }

        false
    }

    fn path(&self, start: Point, goal: Point) -> Vec<(HPoint, PPoint)> {
        let goal = self
            .closed_list
            .iter()
            .find(|(k, _v)| k.pos == goal)
            .unwrap();

        let mut path: Vec<(HPoint, PPoint)> = Vec::new();
        path.push((*goal.0, goal.1.clone()));
        let mut node = goal.0;
        while node.pos != start {
            let parent = self.closed_list[&node].parent.as_ref().unwrap();
            path.push((*parent, self.closed_list[parent].clone()));
            node = &parent;
        }

        path
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut map = Map::new(
        input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect(),
        0,
        3,
    );
    let goal = Point {
        x: map.size.x - 1,
        y: map.size.y - 1,
    };
    let start = (0, 0).into();
    let success = map.a_star((0, 0).into(), goal);

    println!("nodes: {}", map.closed_list.len());

    let path = if success {
        map.path(start, goal)
    } else {
        vec![]
    };

    // for x in 0..map.size.x {
    //     for y in 0..map.size.y {
    //         if let Some((p, _c)) = path.iter().find(|(p, _c)| p.pos.x == x && p.pos.y == y) {
    //             print!("({:>3})", map.closed_list[p].cost);
    //         } else if let Some(p) = map
    //             .closed_list
    //             .iter()
    //             .filter(|(k, _v)| k.pos == (x, y).into())
    //             .min_by_key(|(_k, v)| v.cost)
    //         {
    //             print!(" {:>3} ", p.1.cost);
    //         } else {
    //             print!(" {:>3} ", 'x');
    //         }
    //     }
    //     println!(" ");
    // }
    Ok(path[0].1.cost)
}

fn part2(input: &str) -> Result<usize> {
    let mut map = Map::new(
        input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect(),
        4,
        10,
    );
    let goal = Point {
        x: map.size.x - 1,
        y: map.size.y - 1,
    };
    let start = (0, 0).into();
    let success = map.a_star((0, 0).into(), goal);

    println!("nodes: {}", map.closed_list.len());

    let path = if success {
        map.path(start, goal)
    } else {
        vec![]
    };

    // for x in 0..map.size.x {
    //     for y in 0..map.size.y {
    //         if let Some((p, _c)) = path.iter().find(|(p, _c)| p.pos.x == x && p.pos.y == y) {
    //             print!("({:>3})", map.closed_list[p].cost);
    //         } else if let Some(p) = map
    //             .closed_list
    //             .iter()
    //             .filter(|(k, _v)| k.pos == (x, y).into())
    //             .min_by_key(|(_k, v)| v.cost)
    //         {
    //             print!(" {:>3} ", p.1.cost);
    //         } else {
    //             print!(" {:>3} ", 'x');
    //         }
    //     }
    //     println!(" ");
    // }
    Ok(path[0].1.cost)
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
