use common_lib::get_input;
use std::collections::{HashMap, HashSet};
use std::ops::Index;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct CellIdx {
    pub x: usize,
    pub y: usize,
}

impl From<(usize, usize)> for CellIdx {
    fn from((x, y): (usize, usize)) -> Self {
        CellIdx { x, y }
    }
}

pub struct Plan {
    cells: Vec<Vec<char>>,
    size_x: usize,
    size_y: usize,
}

fn get_offsets() -> Vec<(isize, isize)> {
    vec![
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
    ]
}

impl Plan {
    fn get_neighbours(&self, pos: CellIdx, offsets: Vec<(isize, isize)>) -> Vec<CellIdx> {
        offsets
            .into_iter()
            .filter_map(|(dx, dy)| {
                let (x, y) = (pos.x as isize, pos.y as isize);

                if x + dx >= 0
                    && x + dx < self.size_x as isize
                    && y + dy >= 0
                    && y + dy < self.size_y as isize
                {
                    Some(((x + dx) as usize, (y + dy) as usize).into())
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Index<CellIdx> for Plan {
    type Output = char;

    fn index(&self, pos: CellIdx) -> &Self::Output {
        &self.cells[pos.x][pos.y]
    }
}

impl From<String> for Plan {
    fn from(s: String) -> Self {
        let cells: Vec<Vec<char>> = s.lines().map(|d| d.chars().collect()).collect();
        let size_y = cells.len();
        let size_x = cells[0].len();
        Plan {
            cells,
            size_x,
            size_y,
        }
    }
}

struct Collector {
    numbers: Vec<(u64, Vec<CellIdx>)>,
    current_num: String,
    adjacent: HashSet<CellIdx>,
}

impl Collector {
    fn new() -> Self {
        Collector {
            numbers: Vec::<(u64, Vec<CellIdx>)>::new(),
            current_num: String::new(),
            adjacent: HashSet::new(),
        }
    }

    fn handle_num(&mut self) {
        if let Ok(num) = self.current_num.parse::<u64>() {
            if !self.adjacent.is_empty() {
                let neighbours: Vec<CellIdx> = self.adjacent.drain().collect();
                self.numbers.push((num, neighbours));
                //println!("Valid {num}");
            } else {
                //println!("Invalid {num}");
            }
        }

        self.current_num.clear();
    }

    fn collect(mut self, plan: &Plan) -> Vec<(u64, Vec<CellIdx>)> {
        for i in 0..plan.size_x {
            for j in 0..plan.size_y {
                let cell = (i, j).into();

                if plan[cell].is_ascii_digit() {
                    self.current_num.push(plan[cell]);
                    self.adjacent.extend(
                        plan.get_neighbours(cell, get_offsets())
                            .iter()
                            .filter(|&&c| !plan[c].is_ascii_digit() && plan[c] != '.'),
                    );
                } else {
                    self.handle_num();
                }
            }
            self.handle_num();
        }
        self.numbers
    }
}

fn main() {
    let plan = Plan::from(get_input(3));

    let c = Collector::new();
    let numbers = c.collect(&plan);
    let sum1: u64 = numbers.iter().map(|(n, _)| n).sum();
    println!("Part One {sum1}");

    let mut gears = HashMap::<CellIdx, Vec<u64>>::new();
    numbers.iter().for_each(|(n, cells)| {
        for &c in cells {
            if plan[c] == '*' {
                if let Some(g) = gears.get_mut(&c) {
                    g.push(*n);
                } else {
                    gears.insert(c, vec![*n]);
                }
            }
        }
    });

    let sum2: u64 = gears
        .values()
        .filter(|v| v.len() == 2)
        .map(|v| v[0] * v[1])
        .sum();
    println!("Part Two {sum2}");
}
