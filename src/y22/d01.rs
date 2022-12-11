use crate::io::read_lines;

crate::test::test_part!(test1, part1, ?);
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> usize {
	let mut current_total = 0;
	let mut max_total = 0;
	for line in read_lines("input/2022/01.txt") {
		if let Ok(calories) = line.parse::<usize>() {
			current_total += calories;
		} else {
			max_total = max_total.max(current_total);
			current_total = 0;
		}
	}
	max_total.max(current_total)
}

pub fn part2() -> usize {
	0
}
