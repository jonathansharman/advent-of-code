use crate::io::read_comma_separated_integers;

use itertools::{Itertools, MinMaxResult};

pub fn part1() -> i64 {
	let crabs = read_comma_separated_integers("input/2021/7.txt").collect::<Vec<i64>>();
	crabs
		.iter()
		.map(|crab1| {
			crabs
				.iter()
				.fold(0, |acc, crab2| acc + (crab1 - crab2).abs())
		})
		.min()
		.unwrap()
}

pub fn part2() -> i64 {
	let crabs = read_comma_separated_integers("input/2021/7.txt").collect::<Vec<i64>>();
	let (&min, &max) = match crabs.iter().minmax() {
		MinMaxResult::NoElements => panic!(),
		MinMaxResult::OneElement(only) => (only, only),
		MinMaxResult::MinMax(min, max) => (min, max),
	};
	(min..=max)
		.map(|position| {
			crabs.iter().fold(0, |acc, crab| {
				let distance = (position - crab).abs();
				acc + distance * (distance + 1) / 2
			})
		})
		.min()
		.unwrap()
}

#[cfg(test)]
mod tests {
	#[test]
	fn part1() {
		assert_eq!(343605, super::part1());
	}

	#[test]
	fn part2() {
		assert_eq!(96744904, super::part2());
	}
}
