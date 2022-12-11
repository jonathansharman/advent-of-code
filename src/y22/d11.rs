use std::collections::VecDeque;

use itertools::Itertools;

use crate::io::read_lines;

crate::test::test_part!(test1, part1, 62491);
crate::test::test_part!(test2, part2, 17408399184);

#[derive(Debug)]
enum Op {
	Plus(u64),
	Times(u64),
	Square,
}

#[derive(Debug)]
struct Monkey {
	items: VecDeque<Vec<u64>>,
	op: Op,
	divisor: u64,
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
	let mut lines =
		read_lines("input/2022/11.txt").filter(|line| !line.is_empty());
	while lines.next().is_some() {
		let items = lines
			.next()
			.unwrap()
			.strip_prefix("  Starting items: ")
			.unwrap()
			.split(", ")
			.map(|x| vec![x.parse().unwrap()])
			.collect();
		let op_line = lines.next().unwrap();
		let op = if op_line.ends_with("old") {
			Op::Square
		} else if op_line.contains('*') {
			Op::Times(get_n(&op_line))
		} else {
			Op::Plus(get_n(&op_line))
		};
		let divisor = get_n(&lines.next().unwrap());
		let if_true = get_n(&lines.next().unwrap());
		let if_false = get_n(&lines.next().unwrap());
		monkeys.push(Monkey {
			items,
			op,
			divisor,
			if_true,
			if_false,
			inspections: 0,
		});
	}
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
					Op::Plus(n) => item[0] += n,
					Op::Times(n) => item[0] *= n,
					Op::Square => item[0] *= item[0],
				}
				item[0] /= 3;
				let target = if item[0] % monkeys[idx].divisor == 0 {
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
	let divisors = monkeys.iter().map(|monkey| monkey.divisor).collect_vec();
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
				let target = if item[idx] % monkeys[idx].divisor == 0 {
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

fn apply_op<F>(divisors: &[u64], item: &mut [u64], op: F)
where
	F: Fn(u64) -> u64,
{
	for (i, value) in item.iter_mut().enumerate() {
		*value = op(*value) % divisors[i];
	}
}
