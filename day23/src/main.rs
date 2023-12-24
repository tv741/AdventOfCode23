#![feature(test)]
extern crate test;

use common_lib::{get_input_cached, Point, Result};
use std::collections::{HashMap, HashSet};

use std::fs::File;
use std::io::prelude::*;
use std::thread;

const DAY: usize = 23;

fn build_graph(map: &'_ [Vec<char>], start: Point) -> HashMap<Point, Vec<(Point, usize)>> {
    let mut nodes = HashMap::new();
    _build_graph(
        map,
        Some((0, 1).into()),
        start,
        &mut nodes,
        (0, 1).into(),
        1,
    );

    nodes
}

fn _build_graph(
    map: &'_ [Vec<char>],
    pred: Option<Point>,
    start: Point,
    nodes: &mut HashMap<Point, Vec<(Point, usize)>>,
    node_start: Point,
    curr_len: usize,
) {
    if nodes.contains_key(&node_start) {
        return;
    }

    let offsets = match map[start.x][start.y] {
        '>' => vec![(0, 1)],
        '<' => vec![(0, -1)],
        '^' => vec![(-1, 0)],
        'v' => vec![(1, 0)],
        _ => vec![(0, 1), (-1, 0), (0, -1), (1, 0)],
    };
    let size = Point {
        x: map.len(),
        y: map[0].len(),
    };

    let mut children = vec![];
    for (dx, dy) in offsets.iter() {
        let (x, y) = (start.x as isize, start.y as isize);

        if x + dx >= 0 && x + dx < size.x as isize && y + dy >= 0 && y + dy < size.y as isize {
            let next = Point {
                x: (x + dx) as usize,
                y: (y + dy) as usize,
            };

            if let Some(pred) = pred {
                if pred == next {
                    continue;
                }
            }

            let next_c = map[next.x][next.y];

            if next_c == '.' {
                print!(".");
                _build_graph(map, Some(start), next, nodes, node_start, curr_len + 1);
            } else if match next_c {
                '>' if (dx, dy) == (&0, &1) => true,
                'v' if (dx, dy) == (&1, &0) => true,
                '<' if (dx, dy) == (&0, &-1) => true,
                '^' if (dx, dy) == (&-1, &0) => true,
                _ => false,
            } {
                children.push((next, curr_len));
            }
        } else {
            nodes.insert(
                node_start,
                vec![(
                    Point {
                        x: usize::MAX,
                        y: usize::MAX,
                    },
                    curr_len,
                )],
            );
            nodes.insert(
                Point {
                    x: usize::MAX,
                    y: usize::MAX,
                },
                vec![],
            );
        }
    }

    if children.is_empty() {
        return;
    }

    println!("{:?}", children);
    nodes.insert(node_start, children.clone());

    for (child, _) in children.iter() {
        _build_graph(map, Some(start), *child, nodes, *child, 1);
    }
}

fn draw_graph(graph: &HashMap<Point, Vec<(Point, usize)>>) -> Result<()> {
    let mut file = File::create("day23.txt")?;
    writeln!(file, "digraph {{")?;

    for (node, children) in graph.iter() {
        let name = format!("{}{}", node.x, node.y);
        writeln!(file, "{name} [label=\"{node:?}\"]")?;
        children.iter().for_each(|c| {
            let childs_name = format!("{}{}", c.0.x, c.0.y);
            writeln!(file, "{name} -> {childs_name} [label=\"{}\"]", c.1).unwrap();
        })
    }
    writeln!(file, "}}")?;

    Ok(())
}

fn topo_sort(graph: &HashMap<Point, Vec<(Point, usize)>>, pos: Point) -> Vec<Point> {
    let mut stack = vec![];
    let mut visited = HashSet::new();

    _topo_sort(graph, pos, &mut stack, &mut visited);

    stack
}

fn _topo_sort(
    graph: &HashMap<Point, Vec<(Point, usize)>>,
    pos: Point,
    stack: &mut Vec<Point>,
    visited: &mut HashSet<Point>,
) {
    visited.insert(pos);

    for (p, _) in graph[&pos].iter() {
        if !visited.contains(p) {
            _topo_sort(graph, *p, stack, visited);
        }
    }

    stack.push(pos);
}

fn longest_path(
    graph: &HashMap<Point, Vec<(Point, usize)>>,
    stack: Vec<Point>,
    start: Point,
) -> HashMap<Point, (Option<Point>, isize)> {
    let mut dist: HashMap<Point, (Option<Point>, isize)> =
        HashMap::from_iter(stack.iter().map(|&p| (p, (None, isize::MIN))));
    *dist.get_mut(&start).unwrap() = (None, 0);

    for u in stack.iter().rev() {
        if dist[&u].1 != isize::MAX {
            for (i, dist_i) in graph[u].iter() {
                if dist[&i].1 < dist[u].1 + 1 {
                    *dist.get_mut(i).unwrap() = (Some(*u), dist[u].1 + *dist_i as isize)
                }
            }
        }
    }

    dist
}

fn part1(input: &str) -> Result<usize> {
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let start = (1, 1).into();

    let graph = build_graph(&map, start);
    draw_graph(&graph)?;

    let start = (0, 1).into();
    let stack = topo_sort(&graph, start);
    let res = longest_path(&graph, stack, start);

    let (_pred, max) = res
        .get(&Point {
            x: usize::MAX,
            y: usize::MAX,
        })
        .unwrap();
    Ok(*max as usize)
}

fn part2(_input: &str) -> Result<usize> {
    Ok(0)
}

fn run() {
    let input = get_input_cached(DAY, false).unwrap();
    println!("Part One: {}", part1(&input).unwrap());
}

fn main() -> Result<()> {
    let input = get_input_cached(DAY, false)?;

    // Spawn thread with explicit stack size
    let child = thread::Builder::new()
        .stack_size(u32::MAX as usize)
        .spawn(run)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap();

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
