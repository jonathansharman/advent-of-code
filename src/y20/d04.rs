use itertools::Itertools;

use crate::io::read_lines;
use std::collections::HashMap;

crate::test::test_part!(test1, part1, ?);
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> usize {
	let mut valid_count = 0;
	let mut passport = HashMap::new();
	for line in read_lines("input/2020/04.txt") {
		if line.is_empty() {
			if valid(&passport) {
				valid_count += 1;
			}
			passport = HashMap::new();
		} else {
			passport.extend(
				line.split(' ')
					.map(|kv| kv.split(':').map_into().collect_tuple().unwrap())
					.collect::<HashMap<String, String>>(),
			);
		}
	}
	if valid(&passport) {
		valid_count += 1;
	}
	valid_count
}

pub fn part2() -> usize {
	read_lines("input/2020/04.txt").count()
}

fn valid(passport: &HashMap<String, String>) -> bool {
	["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
		.into_iter()
		.all(|key| passport.contains_key(key))
}
