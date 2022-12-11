use crate::io::read_lines;

crate::test::test_part!(test1, part1, 12458);
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> i32 {
	read_lines("input/2022/02.txt")
		.map(|line| {
			let opp = (line.as_bytes()[0] - b'A') as i32;
			let player = (line.as_bytes()[2] - b'X') as i32;
			let shape_score = player + 1;
			let outcome_score = 3 * ((player - opp + 4) % 3);
			shape_score + outcome_score
		})
		.sum()
}

pub fn part2() -> usize {
	0
}
