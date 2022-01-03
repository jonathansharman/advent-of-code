use crate::io::read_lines;
use itertools::Itertools;

crate::test::test_part!(test1, part1, 636);
crate::test::test_part!(test2, part2, 588);

pub fn part1() -> usize {
	read_lines("input/2020/02.txt")
		.filter(|line| {
			let parts = line.split_whitespace().collect_vec();
			let (min, max) = parts[0]
				.split('-')
				.map(|bound| bound.parse::<usize>().unwrap())
				.collect_tuple()
				.unwrap();
			let letter = parts[1].chars().next().unwrap();
			let password = parts[2];
			let count = password.chars().filter(|c| *c == letter).count();
			min <= count && count <= max
		})
		.count()
}

pub fn part2() -> usize {
	read_lines("input/2020/02.txt")
		.filter(|line| {
			let parts = line.split_whitespace().collect_vec();
			let (first, second) = parts[0]
				.split('-')
				.map(|bound| bound.parse::<usize>().unwrap())
				.collect_tuple()
				.unwrap();
			let letter = parts[1].chars().next().unwrap();
			let password = parts[2];
			(password.chars().nth(first - 1).unwrap() == letter)
				!= (password.chars().nth(second - 1).unwrap() == letter)
		})
		.count()
}
