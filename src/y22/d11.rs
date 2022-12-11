use std::collections::VecDeque;

use crate::io::read_lines;

crate::test::test_part!(test1, part1, 62491);
crate::test::test_part!(test2, part2, ?);

#[derive(Clone, Default, Debug)]
enum Op {
	#[default]
	None,
	Plus(u32),
	Times(u32),
	Square,
}

#[derive(Clone, Default, Debug)]
struct Monkey {
	items: VecDeque<u32>,
	op: Op,
	test_divisor: u32,
	if_true: usize,
	if_false: usize,
	inspections: u32,
}

fn get_n<T: std::str::FromStr>(line: &str) -> T
where
	<T as std::str::FromStr>::Err: std::fmt::Debug,
{
	line.split_whitespace().last().unwrap().parse().unwrap()
}

pub fn part1() -> u32 {
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
				.map(|x| x.parse().unwrap())
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
	for _round in 0..20 {
		for idx in 0..monkeys.len() {
			while !monkeys[idx].items.is_empty() {
				monkeys[idx].inspections += 1;
				let mut item = monkeys[idx].items.pop_front().unwrap();
				match monkeys[idx].op {
					Op::None => (),
					Op::Plus(n) => item += n,
					Op::Times(n) => item *= n,
					Op::Square => item = item.pow(2),
				}
				item /= 3;
				let target = if item % monkeys[idx].test_divisor == 0 {
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

pub fn part2() -> usize {
	0
}
