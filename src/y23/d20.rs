use std::collections::{HashMap, VecDeque};

use crate::io::read_lines;

crate::test::test_part!(test1, part1, 1020211150);
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> usize {
	let mut modules = read_modules();
	let mut low_pulses = 0;
	let mut high_pulses = 0;
	for _ in 0..1000 {
		let mut queue = VecDeque::from([Pulse {
			src: "button".to_owned(), // unused module name
			dst: "broadcaster".to_owned(),
			high: false,
		}]);
		while let Some(Pulse { src, dst, high }) = queue.pop_front() {
			if high {
				high_pulses += 1;
			} else {
				low_pulses += 1;
			}
			let Some(Module { module_type, dsts }) = modules.get_mut(&dst)
			else {
				continue;
			};
			match module_type {
				ModuleType::FlipFlop { on } => {
					if !high {
						*on = !*on;
						let src = dst;
						queue.extend(dsts.iter().cloned().map(|dst| Pulse {
							src: src.clone(),
							dst,
							high: *on,
						}));
					}
				}
				ModuleType::Conjunction { memory } => {
					memory.insert(src, high);
					let src = dst;
					let high = !memory.values().all(|high| *high);
					queue.extend(dsts.iter().cloned().map(|dst| Pulse {
						src: src.clone(),
						dst,
						high,
					}));
				}
				ModuleType::Broadcaster => {
					let src = dst;
					queue.extend(dsts.iter().cloned().map(|dst| Pulse {
						src: src.clone(),
						dst,
						high,
					}))
				}
			}
		}
	}
	low_pulses * high_pulses
}

pub fn part2() -> usize {
	0
}

fn read_modules() -> HashMap<String, Module> {
	let mut modules = HashMap::new();
	for line in read_lines("input/2023/20.txt") {
		let (lhs, rhs) = line.split_once(" -> ").unwrap();
		let dsts: Vec<String> = rhs.split(", ").map(str::to_owned).collect();
		let (name, module_type) = if lhs == "broadcaster" {
			(lhs.to_owned(), ModuleType::Broadcaster)
		} else {
			let (symbol, name) = lhs.split_at(1);
			if symbol == "%" {
				(name.to_owned(), ModuleType::FlipFlop { on: false })
			} else {
				(
					name.to_owned(),
					ModuleType::Conjunction {
						memory: HashMap::new(),
					},
				)
			}
		};
		modules.insert(name, Module { module_type, dsts });
	}
	let mut inputs_by_dst: HashMap<String, Vec<String>> = HashMap::new();
	for (input, module) in &modules {
		for dst in &module.dsts {
			inputs_by_dst
				.entry(dst.clone())
				.or_default()
				.push(input.clone());
		}
	}
	for (dst, i) in inputs_by_dst {
		if let Some(Module {
			module_type: ModuleType::Conjunction { memory },
			..
		}) = modules.get_mut(&dst)
		{
			*memory = i.into_iter().map(|input| (input, false)).collect();
		}
	}
	modules
}

struct Module {
	module_type: ModuleType,
	dsts: Vec<String>,
}

enum ModuleType {
	FlipFlop { on: bool },
	Conjunction { memory: HashMap<String, bool> },
	Broadcaster,
}

struct Pulse {
	src: String,
	dst: String,
	high: bool,
}
