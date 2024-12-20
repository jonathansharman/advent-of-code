use std::collections::HashMap;

aoc::test::test_part!(test1, part1, 19631);
aoc::test::test_part!(test2, part2, 21003205388413);

const INPUT: &str = include_str!("input/08.txt");

#[derive(Clone, Copy)]
enum Direction {
	Left,
	Right,
}

type Network = HashMap<String, (String, String)>;

pub fn part1() -> usize {
	let mut lines = INPUT.lines();
	let directions = lines
		.next()
		.unwrap()
		.chars()
		.map(|c| {
			if c == 'L' {
				Direction::Left
			} else {
				Direction::Right
			}
		})
		.collect::<Vec<_>>();
	let network = Network::from_iter(lines.skip(1).map(|line| {
		let (lhs, rhs) = line.split_once(" = ").unwrap();
		let left = &rhs[1..4];
		let right = &rhs[6..9];
		(lhs.to_owned(), (left.to_owned(), right.to_owned()))
	}));
	let mut symbol = "AAA".to_string();
	for i in 0.. {
		if symbol == "ZZZ" {
			return i;
		}
		let direction = directions[i % directions.len()];
		symbol = match direction {
			Direction::Left => network[&symbol].0.clone(),
			Direction::Right => network[&symbol].1.clone(),
		}
	}
	unreachable!()
}

fn time(
	directions: &[Direction],
	network: &Network,
	mut symbol: String,
) -> usize {
	for i in 0.. {
		if symbol.ends_with('Z') {
			return i;
		}
		let direction = directions[i % directions.len()];
		symbol = match direction {
			Direction::Left => network[&symbol].0.clone(),
			Direction::Right => network[&symbol].1.clone(),
		}
	}
	unreachable!()
}

pub fn part2() -> usize {
	let mut lines = INPUT.lines();
	let directions = lines
		.next()
		.unwrap()
		.chars()
		.map(|c| {
			if c == 'L' {
				Direction::Left
			} else {
				Direction::Right
			}
		})
		.collect::<Vec<_>>();
	let network = HashMap::<String, (String, String)>::from_iter(
		lines.skip(1).map(|line| {
			let (lhs, rhs) = line.split_once(" = ").unwrap();
			let left = &rhs[1..4];
			let right = &rhs[6..9];
			(lhs.to_owned(), (left.to_owned(), right.to_owned()))
		}),
	);
	network
		.keys()
		.filter_map(|symbol| {
			if symbol.ends_with('A') {
				Some(time(&directions, &network, symbol.to_owned()))
			} else {
				None
			}
		})
		.reduce(num::integer::lcm)
		.unwrap()
}
