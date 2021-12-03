use crate::io::read_lines;

use std::collections::HashMap;

pub fn part1() -> i64 {
	let mut n = 0;
	let mut counts = HashMap::new();
	for line in read_lines("input/2021/3-1.txt") {
		n += 1;
		let mut m = u64::from_str_radix(&line, 2).expect("could not parse binary");
		let mut place = 0;
		while m > 0 {
			let count = counts.entry(place).or_insert(0);
			if m & 1 == 1 {
				*count += 1;
			}
			m /= 2;
			place += 1;
		}
	}
	let mut gamma = 0;
	let mut epsilon = 0;
	for (place, count) in counts {
		if count > n / 2 {
			gamma += 1 << place;
		} else {
			epsilon += 1 << place;
		}
	}
	epsilon * gamma
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
