use aoc::input;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

aoc::test::test_part!(test1, part1, 235);
aoc::test::test_part!(test2, part2, 194);

pub fn part1() -> usize {
	count_valid(has_all_fields)
}

pub fn part2() -> usize {
	count_valid(all_fields_valid)
}

type Passport = HashMap<String, String>;

fn count_valid(valid: impl Fn(&Passport) -> bool) -> usize {
	let mut valid_count = 0;
	let mut passport = HashMap::new();
	for line in input!().lines() {
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

fn has_all_fields(passport: &HashMap<String, String>) -> bool {
	["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
		.into_iter()
		.all(|key| passport.contains_key(key))
}

lazy_static! {
	static ref BYR: Regex = Regex::new(r"(^19[2-9]\d|200[0-2]$)").unwrap();
	static ref IYR: Regex = Regex::new(r"(^20(1\d|20)$)").unwrap();
	static ref EYR: Regex = Regex::new(r"(^20(2\d|30)$)").unwrap();
	static ref HGT: Regex =
		Regex::new(r"(^(1([5-8]\d|9[0-3])cm)|((59|6\d|7[0-6])in)$)").unwrap();
	static ref HCL: Regex = Regex::new(r"(^#[0-9a-f]{6}$)").unwrap();
	static ref ECL: Regex =
		Regex::new(r"(^amb|blu|brn|gry|grn|hzl|oth$)").unwrap();
	static ref PID: Regex = Regex::new(r"(^\d{9}$)").unwrap();
}

fn all_fields_valid(passport: &HashMap<String, String>) -> bool {
	passport.get("byr").map(|byr| BYR.is_match(byr)) == Some(true)
		&& passport.get("iyr").map(|iyr| IYR.is_match(iyr)) == Some(true)
		&& passport.get("eyr").map(|eyr| EYR.is_match(eyr)) == Some(true)
		&& passport.get("hgt").map(|hgt| HGT.is_match(hgt)) == Some(true)
		&& passport.get("hcl").map(|hcl| HCL.is_match(hcl)) == Some(true)
		&& passport.get("ecl").map(|ecl| ECL.is_match(ecl)) == Some(true)
		&& passport.get("pid").map(|pid| PID.is_match(pid)) == Some(true)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn byr() {
		assert!(BYR.is_match("1920"));
		assert!(BYR.is_match("2002"));

		assert!(!BYR.is_match("1919"));
		assert!(!BYR.is_match("2003"));
	}

	#[test]
	fn iyr() {
		assert!(IYR.is_match("2010"));
		assert!(IYR.is_match("2020"));

		assert!(!IYR.is_match("2009"));
		assert!(!IYR.is_match("2021"));
	}

	#[test]
	fn eyr() {
		assert!(EYR.is_match("2020"));
		assert!(EYR.is_match("2030"));

		assert!(!EYR.is_match("2019"));
		assert!(!EYR.is_match("2031"));
	}

	#[test]
	fn hgt() {
		assert!(HGT.is_match("150cm"));
		assert!(HGT.is_match("193cm"));
		assert!(HGT.is_match("59in"));
		assert!(HGT.is_match("76in"));

		assert!(!HGT.is_match("149cm"));
		assert!(!HGT.is_match("194cm"));
		assert!(!HGT.is_match("58in"));
		assert!(!HGT.is_match("77in"));

		assert!(!HGT.is_match("193m"));
		assert!(!HGT.is_match("76ft"));
	}

	#[test]
	fn hcl() {
		assert!(HCL.is_match("#123abc"));

		assert!(!HCL.is_match("#123abz"));
		assert!(!HCL.is_match("123abc"));
	}

	#[test]
	fn ecl() {
		assert!(ECL.is_match("brn"));

		assert!(!ECL.is_match("wat"));
	}

	#[test]
	fn pid() {
		assert!(PID.is_match("000000001"));

		assert!(!PID.is_match("0123456789"));
	}
}
