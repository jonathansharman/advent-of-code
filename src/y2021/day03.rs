use crate::{io::read_lines, solution::Solution};

pub struct Day03;

impl Solution for Day03 {
	fn year(&self) -> u32 {
		2021
	}

	fn day(&self) -> u32 {
		3
	}

	fn part1(&self) -> i64 {
		let (bit_len, lines) = {
			let mut lines = read_lines("input/2021/3.txt").peekable();
			let len = lines
				.peek()
				.expect("expected at least one line of input")
				.len();
			(len, lines)
		};
		let mut bit_scores = vec![0; bit_len];
		for line in lines {
			for (bit_idx, bit) in line.bytes().enumerate() {
				match bit {
					b'0' => bit_scores[bit_idx] -= 1,
					b'1' => bit_scores[bit_idx] += 1,
					_ => panic!("expected '0' or '1'"),
				}
			}
		}
		let (mut gamma, mut epsilon) = (0, 0);
		for (idx, score) in bit_scores.into_iter().rev().enumerate() {
			if score > 0 {
				gamma += 1 << idx;
			} else {
				epsilon += 1 << idx;
			}
		}
		gamma * epsilon
	}

	fn part2(&self) -> i64 {
		let mut lines = read_lines("input/2021/3.txt").collect::<Vec<String>>();

		// Sort the lines to enable partitioning by bits at each index.
		lines.sort();

		let oxygen_rating = rating(&lines[..], Criteria::CO2);
		let co2_rating = rating(&lines[..], Criteria::Oxygen);
		oxygen_rating * co2_rating
	}
}

fn rating(mut slice: &[String], criteria: Criteria) -> i64 {
	let mut bit_idx = 0;
	while slice.len() > 1 {
		let partition = slice.partition_point(|element| element.as_bytes()[bit_idx] == b'0');
		let at_least_as_many_ones = partition <= slice.len() / 2;
		slice = match (criteria, at_least_as_many_ones) {
			(Criteria::Oxygen, true) => &slice[partition..],
			(Criteria::Oxygen, false) => &slice[..partition],
			(Criteria::CO2, true) => &slice[..partition],
			(Criteria::CO2, false) => &slice[partition..],
		};
		bit_idx += 1;
	}
	i64::from_str_radix(&slice[0], 2).expect("could not parse binary")
}

#[derive(Clone, Copy)]
enum Criteria {
	Oxygen,
	CO2,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1() {
		assert_eq!(841526, Day03.part1());
	}

	#[test]
	fn part2() {
		assert_eq!(4790390, Day03.part2());
	}
}
