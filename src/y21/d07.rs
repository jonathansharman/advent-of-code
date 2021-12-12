use crate::io::read_comma_separated_integers;

use itertools::{Itertools, MinMaxResult};

crate::test::test_part!(test1, part1, 343605);
crate::test::test_part!(test2, part2, 96744904);

pub fn part1() -> i64 {
	let crabs = read_comma_separated_integers("input/2021/07.txt").collect::<Vec<i64>>();
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
	let crabs = read_comma_separated_integers("input/2021/07.txt").collect::<Vec<i64>>();
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
