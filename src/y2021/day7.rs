use crate::{io::read_comma_separated_integers, solution::Solution};

use itertools::{Itertools, MinMaxResult};

pub struct Day7;

impl Solution for Day7 {
	fn year(&self) -> u32 {
		2021
	}

	fn day(&self) -> u32 {
		7
	}

	fn part1(&self) -> i64 {
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

	fn part2(&self) -> i64 {
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
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1() {
		assert_eq!(343605, Day7.part1());
	}

	#[test]
	fn part2() {
		assert_eq!(96744904, Day7.part2());
	}
}
