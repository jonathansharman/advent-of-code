use std::collections::{HashMap, VecDeque};

aoc::test::test_part!(test1, part1, 1020211150);
aoc::test::test_part!(test2, part2, 238815727638557);

const INPUT: &str = include_str!("input/20.txt");

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
			let Some((module, dsts)) = modules.get_mut(&dst) else {
				continue;
			};
			match module {
				Module::FlipFlop { on } => {
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
				Module::Conjunction { memory } => {
					memory.insert(src, high);
					let src = dst;
					let high = !memory.values().all(|high| *high);
					queue.extend(dsts.iter().cloned().map(|dst| Pulse {
						src: src.clone(),
						dst,
						high,
					}));
				}
				Module::Broadcaster => {
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
	let modules = read_modules();
	modules["broadcaster"]
		.1
		.iter()
		.cloned()
		.map(|dst| period(&modules, dst))
		.product()
}

fn period(
	modules: &HashMap<String, (Module, Vec<String>)>,
	mut name: String,
) -> usize {
	let mut result = 0;
	for pow in 0.. {
		let mut bit = 0;
		let dsts = &modules[&name].1;
		let mut found_next = false;
		for dst in dsts {
			let module = &modules[dst].0;
			match module {
				Module::FlipFlop { .. } => {
					found_next = true;
					name = dst.clone()
				}
				Module::Conjunction { .. } => bit = 1,
				_ => {}
			}
		}
		result += bit << pow;
		if !found_next {
			break;
		}
	}
	result
}

/// module name -> (module, destination module names)
fn read_modules() -> HashMap<String, (Module, Vec<String>)> {
	let mut modules = HashMap::new();
	for line in INPUT.lines() {
		let (lhs, rhs) = line.split_once(" -> ").unwrap();
		let dsts: Vec<String> = rhs.split(", ").map(str::to_owned).collect();
		let (name, module) = if lhs == "broadcaster" {
			(lhs.to_owned(), Module::Broadcaster)
		} else {
			let (symbol, name) = lhs.split_at(1);
			if symbol == "%" {
				(name.to_owned(), Module::FlipFlop { on: false })
			} else {
				(
					name.to_owned(),
					Module::Conjunction {
						memory: HashMap::new(),
					},
				)
			}
		};
		modules.insert(name, (module, dsts));
	}
	let mut inputs_by_dst: HashMap<String, Vec<String>> = HashMap::new();
	for (input, (_, dsts)) in &modules {
		for dst in dsts {
			inputs_by_dst
				.entry(dst.clone())
				.or_default()
				.push(input.clone());
		}
	}
	for (dst, i) in inputs_by_dst {
		if let Some((Module::Conjunction { memory }, _)) = modules.get_mut(&dst)
		{
			*memory = i.into_iter().map(|input| (input, false)).collect();
		}
	}
	modules
}

enum Module {
	FlipFlop { on: bool },
	Conjunction { memory: HashMap<String, bool> },
	Broadcaster,
}

struct Pulse {
	src: String,
	dst: String,
	high: bool,
}
