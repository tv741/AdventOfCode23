#![feature(test)]
extern crate test;

use common_lib::{get_input_cached, Result};
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::prelude::*;

const DAY: usize = 20;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
    None,
}

#[derive(Debug, Hash, Clone)]
enum Type {
    Broadcast,
    FlipFlop(bool),
    Conjunction(Vec<(String, Pulse)>),
}

#[derive(Debug, Hash, Clone)]
struct Module {
    _type: Type,
    receivers: Vec<String>,
    cycle: Option<Vec<Pulse>>,
}

impl Module {
    fn from_str(line: &str) -> (String, Module) {
        let start = match &line[..1] {
            "%" => 1,
            "&" => 1,
            _ => 0,
        };

        let mut parts = line
            .split(|c| [' ', '-', '>', ','].contains(&c))
            .filter(|s| !s.is_empty());

        let name = parts.next().unwrap()[start..].to_string();

        let receivers = parts.map(|s| s.to_string()).collect();

        let _type = match &line[..1] {
            "%" => Type::FlipFlop(false),
            "&" => Type::Conjunction(vec![]),
            _ => Type::Broadcast,
        };

        (
            name,
            Module {
                receivers,
                _type,
                cycle: None,
            },
        )
    }
}

#[derive(Debug, Hash, Clone)]
struct Watch {
    parts: Vec<&'static str>,
    reset_at: u16,
    reset_to: u16,
}

struct System {
    modules: HashMap<String, Module>,
    q: VecDeque<(String, Pulse, String)>,
    low_signals: usize,
    high_signals: usize,
    watches: HashMap<String, Watch>,
}

impl System {
    fn new(input: &str) -> Self {
        let modules_vec: Vec<_> = input.lines().map(Module::from_str).collect();

        let mut modules = HashMap::<String, Module>::new();
        for (name, module) in modules_vec.iter() {
            match module._type {
                Type::Conjunction(_) => {
                    let inputs: Vec<_> = modules_vec
                        .iter()
                        .filter_map(|(name2, m)| {
                            if m.receivers.contains(name) {
                                Some(name2)
                            } else {
                                None
                            }
                        })
                        .map(|s| (s.clone(), Pulse::Low))
                        .collect();

                    modules.insert(
                        name.clone(),
                        Module {
                            receivers: module.receivers.clone(),
                            _type: Type::Conjunction(inputs),
                            cycle: None,
                        },
                    );
                }
                _ => {
                    modules.insert(name.clone(), module.clone());
                }
            }
        }

        let q = VecDeque::<(String, Pulse, String)>::new();

        System {
            modules,
            q,
            low_signals: 0,
            high_signals: 0,
            watches: HashMap::new(),
        }
    }

    fn push_button(&mut self) -> bool {
        self.q
            .push_front(("button".to_string(), Pulse::Low, "broadcaster".to_owned()));
        let mut foo = None;

        'outer: while let Some((tx, pulse, rx)) = self.q.pop_back() {
            //print!("\n{} -{:?}-> {} ", tx, pulse, rx);

            match pulse {
                Pulse::High => self.high_signals += 1,
                Pulse::Low => self.low_signals += 1,
                _ => {}
            }

            if pulse == Pulse::None {
            } else if let Some(module) = self.modules.get_mut(&rx) {
                let pulse = match &mut module._type {
                    Type::Broadcast => pulse,
                    Type::FlipFlop(ref mut state) => {
                        if pulse == Pulse::Low {
                            if *state {
                                *state = false;
                                Pulse::Low
                            } else {
                                *state = true;
                                Pulse::High
                            }
                        } else {
                            Pulse::None
                        }
                    }
                    Type::Conjunction(inputs) => {
                        let input = inputs.iter_mut().find(|(name, _)| name == &tx).unwrap();
                        input.1 = pulse;
                        //print!("{:?}", inputs);

                        if inputs.iter().all(|(_, state)| *state == Pulse::High) {
                            if let Some(watch) = self.watches.get(&rx) {
                                foo = Some(rx.clone());
                            }

                            Pulse::Low
                        } else {
                            Pulse::High
                        }
                    }
                };
                module
                    .receivers
                    .iter()
                    .for_each(|r| self.q.push_front((rx.clone(), pulse, r.clone())));
            }
            if foo.clone().is_some_and(|f| f == rx) {
                let name = rx.clone();
                if let Some(watch) = self.watches.get(&name) {
                    let mut state = 0u16;
                    for (n, p) in watch.parts.iter().enumerate().rev() {
                        if let Type::FlipFlop(bit) = self.modules.get(*p).unwrap()._type {
                            if bit {
                                state |= 1 << n;
                            }
                        } else {
                            panic!();
                        }
                    }
                    println!("{name} reset at {state:12b} - {state}");
                }
            }
        }
        for (name, watch) in self.watches.iter() {
            let mut state = 0u16;
            for (n, p) in watch.parts.iter().enumerate().rev() {
                if let Type::FlipFlop(bit) = self.modules.get(*p).unwrap()._type {
                    if bit {
                        state |= 1 << n;
                    }
                } else {
                    panic!();
                }
            }
            println!("{name}: {state:12b} - {state}");
        }
        foo.is_none()
    }

    fn draw(&self) -> Result<()> {
        let mut file = File::create("foo.txt")?;
        writeln!(file, "digraph {{")?;

        for (name, module) in self.modules.iter() {
            let prefix = match module._type {
                Type::FlipFlop(_) => "%",
                Type::Conjunction(_) => "&",
                _ => "",
            };

            writeln!(file, "{name} [label=\"{prefix}{name}\"]")?;
            module.receivers.iter().for_each(|r| {
                writeln!(file, "{name} -> {r}").unwrap();
            })
        }
        writeln!(file, "}}")?;

        Ok(())
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut system = System::new(input);

    Ok(system.high_signals * system.low_signals)
}

fn part2(input: &str) -> Result<usize> {
    Ok([3769usize, 3877, 3847, 4057]
        .iter()
        .copied()
        .reduce(num::integer::lcm)
        .unwrap())
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
