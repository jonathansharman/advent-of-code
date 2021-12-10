use crate::{io::read_lines, solution::Solution};

use itertools::Itertools;

pub struct Day02;

impl Solution for Day02 {
	fn year(&self) -> u32 {
		2021
	}

	fn day(&self) -> u32 {
		2
	}

	fn part1(&self) -> i64 {
		let (depth, latitude) = read_lines("input/2021/2.txt")
			.map(|line| {
				let (direction, distance_string) = line
					.split(' ')
					.collect_tuple()
					.expect("expected exactly two elements");
				let distance = distance_string
					.parse::<i64>()
					.expect("could not parse distance");
				(direction.to_owned(), distance)
			})
			.fold(
				(0, 0),
				|(depth, latitude), (direction, distance)| match direction.as_str() {
					"forward" => (depth, latitude + distance),
					"down" => (depth + distance, latitude),
					"up" => (depth - distance, latitude),
					_ => panic!("unexpected direction: {}", direction),
				},
			);
		depth * latitude
	}

	fn part2(&self) -> i64 {
		let (depth, latitude, _) = read_lines("input/2021/2.txt")
			.map(|line| {
				let (direction, distance_string) = line
					.split(' ')
					.collect_tuple()
					.expect("expected exactly two elements");
				let distance = distance_string
					.parse::<i64>()
					.expect("could not parse distance");
				(direction.to_owned(), distance)
			})
			.fold(
				(0, 0, 0),
				|(depth, latitude, aim), (direction, distance)| match direction.as_str() {
					"forward" => (depth + aim * distance, latitude + distance, aim),
					"down" => (depth, latitude, aim + distance),
					"up" => (depth, latitude, aim - distance),
					_ => panic!("unexpected direction: {}", direction),
				},
			);
		depth * latitude
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1() {
		assert_eq!(2187380, Day02.part1());
	}

	#[test]
	fn part2() {
		assert_eq!(2086357770, Day02.part2());
	}
}
