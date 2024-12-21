aoc::test::test_part!(test1, part1, 12458);
aoc::test::test_part!(test2, part2, 12683);

const INPUT: &str = include_str!("input.txt");

pub fn part1() -> i32 {
	INPUT
		.lines()
		.map(|line| {
			let opp = (line.as_bytes()[0] - b'A') as i32;
			let player = (line.as_bytes()[2] - b'X') as i32;
			let shape_score = player + 1;
			let outcome_score = 3 * ((player - opp + 1 + 3) % 3);
			shape_score + outcome_score
		})
		.sum()
}

pub fn part2() -> i32 {
	INPUT
		.lines()
		.map(|line| {
			let opp = (line.as_bytes()[0] - b'A') as i32;
			let goal = (line.as_bytes()[2] - b'X') as i32;
			let player = (opp + (goal - 1) + 3) % 3;
			let shape_score = player + 1;
			let outcome_score = 3 * ((player - opp + 1 + 3) % 3);
			shape_score + outcome_score
		})
		.sum()
}
