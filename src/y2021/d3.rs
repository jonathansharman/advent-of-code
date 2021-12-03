use crate::io::read_lines;

pub fn part1() -> i64 {
	let (bit_len, lines) = {
		let mut lines = read_lines("input/2021/3-1.txt").peekable();
		let len = lines
			.peek()
			.expect("expected at least one line of input")
			.len();
		(len, lines)
	};
	let mut bit_scores = vec![0; bit_len];
	for line in lines {
		for (idx, bit) in line.bytes().enumerate() {
			match bit {
				b'0' => bit_scores[idx] -= 1,
				b'1' => bit_scores[idx] += 1,
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

pub fn part2() -> i64 {
	let lines = read_lines("input/2021/3-2.txt").collect::<Vec<String>>();
	let oxygen_rating = rating(lines.clone(), Criteria::LeastCommon);
	let co2_rating = rating(lines, Criteria::MostCommon);
	oxygen_rating * co2_rating
}

#[derive(Clone, Copy)]
enum Criteria {
	MostCommon,
	LeastCommon,
}

fn rating(mut lines: Vec<String>, criteria: Criteria) -> i64 {
	let mut idx = 0;
	while lines.len() > 1 {
		let target = get_target(&lines, idx, criteria);
		lines = lines
			.into_iter()
			.filter(|line| line.chars().nth(idx).unwrap() == target)
			.collect();
		idx += 1;
	}
	i64::from_str_radix(&lines[0], 2).expect("could not parse binary")
}

fn get_target(lines: &[String], idx: usize, criteria: Criteria) -> char {
	let mut zeros = 0;
	let mut ones = 0;
	for line in lines {
		match line.chars().nth(idx).unwrap() {
			'0' => zeros += 1,
			'1' => ones += 1,
			_ => panic!("non-binary digit"),
		}
	}
	use std::cmp::Ordering::*;
	use Criteria::*;
	match (criteria, zeros.cmp(&ones)) {
		(LeastCommon, Greater) => '1',
		(LeastCommon, Less) => '0',
		(LeastCommon, Equal) => '0',
		(MostCommon, Greater) => '0',
		(MostCommon, Less) => '1',
		(MostCommon, Equal) => '1',
	}
}
