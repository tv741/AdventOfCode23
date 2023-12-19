#![feature(test)]
extern crate test;

use common_lib::{get_input_cached, Result};
use std::collections::{HashMap, VecDeque};

const DAY: usize = 20;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Hash, Clone)]
enum Module {
    Broadcast(Vec<String>),
    FlipFlop(bool, Vec<String>),
    Conjunction(Vec<(String, Pulse)>, Vec<String>),
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

        let module = match &line[..1] {
            "%" => Module::FlipFlop(false, receivers),
            "&" => Module::Conjunction(vec![], receivers),
            _ => Module::Broadcast(receivers),
        };

        (name, module)
    }
}

struct System {
    modules: HashMap<String, Module>,
    q: VecDeque<(String, Pulse, String)>,
    low_signals: usize,
    high_signals: usize,
    rx: Option<usize>,
}

impl System {
    fn new(input: &str) -> Self {
        let modules_vec: Vec<_> = input.lines().map(|l| Module::from_str(l)).collect();

        let mut modules = HashMap::<String, Module>::new();
        for (name, module) in modules_vec.iter() {
            match module {
                Module::Conjunction(_, output) => {
                    let inputs: Vec<_> = modules_vec
                        .iter()
                        .filter_map(|(name2, m)| match m {
                            Module::Broadcast(outputs) => {
                                if outputs.contains(&name) {
                                    Some(name2)
                                } else {
                                    None
                                }
                            }
                            Module::FlipFlop(_, outputs) => {
                                if outputs.contains(&name) {
                                    Some(name2)
                                } else {
                                    None
                                }
                            }
                            Module::Conjunction(_, outputs) => {
                                if outputs.contains(&name) {
                                    Some(name2)
                                } else {
                                    None
                                }
                            }
                        })
                        .map(|s| (s.clone(), Pulse::Low))
                        .collect();

                    modules.insert(name.clone(), Module::Conjunction(inputs, output.clone()));
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
            rx: None,
        }
    }

    fn push_button(&mut self) {
        self.q
            .push_front(("button".to_string(), Pulse::Low, "broadcaster".to_owned()));

        while let Some((tx, pulse, rx)) = self.q.pop_back() {
            //print!("\n{} -{:?}-> {} ", tx, pulse, rx);

            if pulse == Pulse::High {
                self.high_signals += 1;
            } else {
                self.low_signals += 1;
            }

            if pulse == Pulse::Low && &rx == "rx" {
                self.rx = Some(self.high_signals + self.low_signals);
            }

            if let Some(module) = self.modules.get_mut(&rx) {
                match module {
                    Module::Broadcast(receivers) => receivers
                        .iter()
                        .for_each(|r| self.q.push_front((rx.clone(), pulse, r.clone()))),
                    Module::FlipFlop(ref mut state, receivers) => {
                        if pulse == Pulse::Low {
                            if *state {
                                *state = false;
                                receivers.iter().for_each(|r| {
                                    self.q.push_front((rx.clone(), Pulse::Low, r.clone()))
                                });
                            } else {
                                *state = true;
                                receivers.iter().for_each(|r| {
                                    self.q.push_front((rx.clone(), Pulse::High, r.clone()))
                                });
                            }
                        }
                    }
                    Module::Conjunction(inputs, receivers) => {
                        let input = inputs.iter_mut().find(|(name, _)| name == &tx).unwrap();
                        input.1 = pulse;
                        //print!("{:?}", inputs);
                        if inputs.iter().all(|(_, state)| *state == Pulse::High) {
                            receivers.iter().for_each(|r| {
                                self.q.push_front((rx.clone(), Pulse::Low, r.clone()))
                            });
                        } else {
                            receivers.iter().for_each(|r| {
                                self.q.push_front((rx.clone(), Pulse::High, r.clone()))
                            });
                        }
                    }
                }
            }
        }
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut system = System::new(input);

    for _ in 0..1000 {
        system.push_button();
        //println!("")
    }
    println!("Part Two: {:?}", system.rx);
    Ok(system.high_signals * system.low_signals)
}

fn part2(input: &str) -> Result<usize> {
    let mut system = System::new(input);

    while system.rx.is_none() {
        system.push_button();
        //println!("")
    }

    Ok(system.rx.unwrap())
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
