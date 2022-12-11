use std::collections::VecDeque;

use itertools::Itertools;

use crate::io::read_lines;

crate::test::test_part!(test1, part1, 62491);
crate::test::test_part!(test2, part2, 17408399184);

#[derive(Clone, Default, Debug)]
enum Op {
	#[default]
	None,
	Plus(u64),
	Times(u64),
	Square,
}

#[derive(Clone, Default, Debug)]
struct Monkey {
	items: VecDeque<Vec<u64>>,
	op: Op,
	test_divisor: u64,
	if_true: usize,
	if_false: usize,
	inspections: u64,
}

fn get_n<T: std::str::FromStr>(line: &str) -> T
where
	<T as std::str::FromStr>::Err: std::fmt::Debug,
{
	line.split_whitespace().last().unwrap().parse().unwrap()
}

fn read_monkeys() -> Vec<Monkey> {
	let mut monkeys = Vec::new();
	let mut next_monkey = Monkey::default();
	for line in read_lines("input/2022/11.txt") {
		if line.is_empty() {
			monkeys.push(next_monkey.clone());
		} else if line.starts_with("  Starting items: ") {
			next_monkey.items = line
				.strip_prefix("  Starting items: ")
				.unwrap()
				.split(", ")
				.map(|x| vec![x.parse().unwrap()])
				.collect();
		} else if line.starts_with("  Operation") {
			next_monkey.op = if line.contains('*') {
				if line.ends_with("old") {
					Op::Square
				} else {
					Op::Times(get_n(&line))
				}
			} else {
				Op::Plus(get_n(&line))
			};
		} else if line.starts_with("  Test") {
			next_monkey.test_divisor = get_n(&line);
		} else if line.starts_with("    If true") {
			next_monkey.if_true = get_n(&line);
		} else if line.starts_with("    If false") {
			next_monkey.if_false = get_n(&line);
		}
	}
	monkeys.push(next_monkey);
	monkeys
}

pub fn part1() -> u64 {
	let mut monkeys = read_monkeys();
	for _round in 0..20 {
		for idx in 0..monkeys.len() {
			while !monkeys[idx].items.is_empty() {
				monkeys[idx].inspections += 1;
				let mut item = monkeys[idx].items.pop_front().unwrap();
				match monkeys[idx].op {
					Op::None => (),
					Op::Plus(n) => item[0] += n,
					Op::Times(n) => item[0] *= n,
					Op::Square => item[0] *= item[0],
				}
				item[0] /= 3;
				let target = if item[0] % monkeys[idx].test_divisor == 0 {
					monkeys[idx].if_true
				} else {
					monkeys[idx].if_false
				};
				monkeys[target].items.push_back(item);
			}
		}
	}
	monkeys.sort_by(|m1, m2| m2.inspections.cmp(&m1.inspections));
	monkeys[0].inspections * monkeys[1].inspections
}

pub fn part2() -> u64 {
	let mut monkeys = read_monkeys();
	let divisors = monkeys
		.iter()
		.map(|monkey| monkey.test_divisor)
		.collect_vec();
	for monkey in monkeys.iter_mut() {
		for item in monkey.items.iter_mut() {
			*item = divisors.iter().map(|d| item[0] % d).collect();
		}
	}
	for _round in 0..10_000 {
		for idx in 0..monkeys.len() {
			while !monkeys[idx].items.is_empty() {
				monkeys[idx].inspections += 1;
				let mut item = monkeys[idx].items.pop_front().unwrap();
				match monkeys[idx].op {
					Op::None => (),
					Op::Plus(n) => apply_op(&divisors, &mut item, |x| {
						x.checked_add(n).unwrap()
					}),
					Op::Times(n) => apply_op(&divisors, &mut item, |x| {
						x.checked_mul(n).unwrap()
					}),
					Op::Square => apply_op(&divisors, &mut item, |x| {
						x.checked_mul(x).unwrap()
					}),
				}
				let target = if item[idx] % monkeys[idx].test_divisor == 0 {
					monkeys[idx].if_true
				} else {
					monkeys[idx].if_false
				};
				monkeys[target].items.push_back(item);
			}
		}
	}
	monkeys.sort_by(|m1, m2| m2.inspections.cmp(&m1.inspections));
	//print_inspections(0, &monkeys);
	monkeys[0].inspections * monkeys[1].inspections
}

fn apply_op<F>(divisors: &[u64], item: &mut [u64], op: F)
where
	F: Fn(u64) -> u64,
{
	for (i, value) in item.iter_mut().enumerate() {
		*value = op(*value) % divisors[i];
	}
}

fn print_inspections(round: usize, monkeys: &[Monkey]) {
	let is: Vec<_> =
		monkeys.iter().map(|m| m.inspections).enumerate().collect();
	println!("{round}: {is:?}");
}
