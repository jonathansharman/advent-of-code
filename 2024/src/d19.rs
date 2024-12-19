use aoc::io::read_lines;
use itertools::Itertools;
use regex::Regex;

aoc::test::test_part!(test1, part1, 340);
aoc::test::test_part!(test2, part2, ?);

pub fn part1() -> usize {
	let mut lines = read_lines("input/19.txt");
	let patterns = lines.next().unwrap().split(", ").join("|");
	let regex = Regex::new(&format!("^({patterns})+$",)).unwrap();
	lines.skip(1).filter(|towel| regex.is_match(towel)).count()
}

pub fn part2() -> usize {
	0
}
