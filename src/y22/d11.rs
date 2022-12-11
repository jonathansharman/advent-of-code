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
	items: VecDeque<usize>,
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

fn read_monkeys_and_worries() -> (Vec<Monkey>, Vec<u64>) {
	let mut monkeys = Vec::new();
	let mut worries: Vec<u64> = Vec::new();
	let mut lines =
		read_lines("input/2022/11.txt").filter(|line| !line.is_empty());
	while lines.next().is_some() {
		let n = worries.len();
		worries.append(
			&mut lines
				.next()
				.unwrap()
				.strip_prefix("  Starting items: ")
				.unwrap()
				.split(", ")
				.map(|worry| worry.parse().unwrap())
				.collect(),
		);
		let items = (n..worries.len()).collect();
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
	(monkeys, worries)
}

pub fn part1() -> u64 {
	solve(true)
}

pub fn part2() -> u64 {
	solve(false)
}

fn solve(part1: bool) -> u64 {
	let (mut monkeys, worries) = read_monkeys_and_worries();
	// (worry, modulus) pairs
	let mut worry_map = worries
		.into_iter()
		.map(|worry| {
			monkeys
				.iter()
				.map(|monkey| (worry, monkey.divisor))
				.collect_vec()
		})
		.collect_vec();
	let rounds = if part1 { 20 } else { 10_000 };
	for _ in 0..rounds {
		for m in 0..monkeys.len() {
			while !monkeys[m].items.is_empty() {
				monkeys[m].inspections += 1;
				let i = monkeys[m].items.pop_front().unwrap();
				for (worry, modulus) in worry_map[i].iter_mut() {
					match monkeys[m].op {
						Op::Plus(n) => *worry += n,
						Op::Times(n) => *worry *= n,
						Op::Square => *worry *= *worry,
					}
					if part1 {
						*worry /= 3;
					} else {
						*worry %= *modulus;
					}
				}
				let target = if worry_map[i][m].0 % monkeys[m].divisor == 0 {
					monkeys[m].if_true
				} else {
					monkeys[m].if_false
				};
				monkeys[target].items.push_back(i);
			}
		}
	}
	monkeys.sort_by(|m1, m2| m2.inspections.cmp(&m1.inspections));
	monkeys[0].inspections * monkeys[1].inspections
}
