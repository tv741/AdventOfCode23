#![feature(test)]
extern crate test;

use std::cell::RefCell;
use std::collections::HashMap;

use common_lib::{get_input_cached, Result};

const DAY: usize = 19;

struct Part {
    ratings: HashMap<char, usize>,
}

type Workflow<'a> = Vec<&'a str>;
struct Machine<'a> {
    workflows: HashMap<String, Workflow<'a>>,
    unsorted: Vec<Part>,
    accepted: Vec<Part>,
    rejected: Vec<Part>,
}

impl<'a> Machine<'a> {
    fn new(input: &'a str) -> Self {
        let mut lines = input.lines();
        let workflows = HashMap::from_iter(lines.by_ref().take_while(|l| !l.is_empty()).map(|l| {
            let mut parts = l.split(|c| ['{', ',', '}'].contains(&c));
            let name = parts.next().unwrap();
            let workflow: Workflow = parts.collect();
            (name.to_owned(), workflow)
        }));
        let unsorted = lines
            .map(|l| {
                let parts = l.split(|c| ['{', ',', '}'].contains(&c));
                let ratings = HashMap::from_iter(
                    parts
                        .filter(|p| !p.is_empty())
                        .map(|s| (s.chars().next().unwrap(), s[2..].parse::<usize>().unwrap())),
                );
                Part { ratings }
            })
            .collect();
        let accepted = Vec::new();
        let rejected = Vec::new();

        Machine {
            workflows,
            unsorted,
            accepted,
            rejected,
        }
    }

    fn sort(&mut self) {
        while let Some(part) = self.unsorted.pop() {
            print!("\nin");
            self.apply_workflow(part, "in");
        }
        println!("");
    }

    fn apply_workflow(&mut self, part: Part, workflow: &str) {
        let workflow = self.workflows.get(workflow).unwrap().clone();
        let mut part = part;
        for rule in workflow.iter() {
            if let Some(p) = self.apply_rule(part, rule) {
                part = p;
            } else {
                break;
            }
        }
    }

    fn apply_rule(&mut self, part: Part, rule: &str) -> Option<Part> {
        let parts = rule.split(':').collect::<Vec<_>>();
        let (applies, dst) = if parts.len() == 2 {
            let mut chars = rule.chars();
            let rating = part.ratings[&chars.next().unwrap()];
            let op = chars.next().unwrap();
            let val = parts[0][2..].parse::<usize>().unwrap();

            (
                match op {
                    '<' => rating < val,
                    '>' => rating > val,
                    _ => panic!(),
                },
                parts[1],
            )
        } else {
            (true, parts[0])
        };

        if applies {
            print!(" -> {dst}");
            match dst {
                "A" => self.accepted.push(part),
                "R" => self.rejected.push(part),
                workflow => self.apply_workflow(part, workflow),
            };
            None
        } else {
            Some(part)
        }
    }

    fn score(&self) -> usize {
        let mut total = 0;
        for part in self.accepted.iter() {
            total += part.ratings.values().sum::<usize>();
        }
        total
    }
}

#[derive(Clone, Debug)]
struct PartRange {
    ratings: HashMap<char, (usize, usize)>,
}

impl PartRange {
    fn new() -> Self {
        let ratings = HashMap::from([
            ('x', (1, 4000)),
            ('m', (1, 4000)),
            ('a', (1, 4000)),
            ('s', (1, 4000)),
        ]);
        Self { ratings }
    }

    fn count(&self) -> usize {
        self.ratings
            .values()
            .map(|(min, max)| max - min + 1)
            .fold(1, |acc, v| acc * v)
    }
}

struct MachineRange<'a> {
    workflows: HashMap<String, Workflow<'a>>,
}

impl<'a> MachineRange<'a> {
    fn new(input: &'a str) -> Self {
        let mut lines = input.lines();
        let workflows = HashMap::from_iter(lines.by_ref().take_while(|l| !l.is_empty()).map(|l| {
            let mut parts = l.split(|c| ['{', ',', '}'].contains(&c));
            let name = parts.next().unwrap();
            let workflow: Workflow = parts.collect();
            (name.to_owned(), workflow)
        }));

        MachineRange { workflows }
    }

    fn sort(&mut self) -> usize {
        println!("\n -> in {:?}\n", PartRange::new().ratings,);
        self.apply_workflow(PartRange::new(), "in")
    }

    fn apply_workflow(&mut self, part: PartRange, workflow: &str) -> usize {
        let workflow = self.workflows.get(workflow).unwrap().clone();
        let mut part = part;
        let mut total = 0;
        for rule in workflow.iter() {
            let (count, op) = self.apply_rule(part, rule);
            total += count;
            if let Some(p) = op {
                part = p;
            } else {
                break;
            }
        }
        total
    }

    fn apply_rule(&mut self, mut part: PartRange, rule: &str) -> (usize, Option<PartRange>) {
        let parts = rule.split(':').collect::<Vec<_>>();
        let (applies, dst, rest) = if parts.len() == 2 {
            let mut chars = rule.chars();
            //let rating = part.ratings[&chars.next().unwrap()];
            let rating = chars.next().unwrap();

            let op = chars.next().unwrap();
            let val = parts[0][2..].parse::<usize>().unwrap();

            let mut rest = part.clone();

            let (min, max) = part.ratings[&rating];

            match op {
                '<' => {
                    *part.ratings.get_mut(&rating).unwrap() = (min, val.min(max) - 1);
                    *rest.ratings.get_mut(&rating).unwrap() = (val.min(max), max);
                }
                '>' => {
                    *part.ratings.get_mut(&rating).unwrap() = (val.max(min) + 1, max);
                    *rest.ratings.get_mut(&rating).unwrap() = (min, val.max(min));
                }
                _ => panic!(),
            }

            println!(
                "\n{rating} {op} {val:4} -> {} {:?}\n        Rest: {:?}",
                parts[1], part.ratings, rest.ratings
            );

            (part, parts[1], Some(rest))
        } else {
            println!("\ndefault -> {} {:?}", parts[0], part.ratings);
            (part, parts[0], None)
        };

        let count = match dst {
            "A" => applies.count(),
            "R" => 0,
            workflow => self.apply_workflow(applies, workflow),
        };

        (count, rest)
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut m = Machine::new(input);
    m.sort();

    Ok(m.score())
}

fn part2(input: &str) -> Result<usize> {
    let mut m = MachineRange::new(input);
    let total = m.sort();

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
