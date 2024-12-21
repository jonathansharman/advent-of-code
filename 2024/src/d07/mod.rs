aoc::test::test_part!(test1, part1, 882304362421);
aoc::test::test_part!(test2, part2, 145149066755184);

const INPUT: &str = include_str!("input.txt");

fn parse_equation(value: &str) -> (i64, Vec<i64>) {
	let (lhs, rhs) = value.split_once(": ").unwrap();
	(
		lhs.parse().unwrap(),
		rhs.split(' ').map(|n| n.parse().unwrap()).collect(),
	)
}

fn has_solution(lhs: i64, rhs: &[i64]) -> bool {
	let (&last, rest) = rhs.split_last().unwrap();
	if rest.is_empty() {
		lhs == last
	} else {
		has_solution(lhs - last, rest)
			|| (lhs % last == 0 && has_solution(lhs / last, rest))
	}
}

pub fn part1() -> i64 {
	INPUT
		.lines()
		.filter_map(|line| {
			let (lhs, rhs) = parse_equation(line);
			has_solution(lhs, &rhs).then_some(lhs)
		})
		.sum()
}

fn has_solution_2(lhs: i64, rhs: &[i64]) -> bool {
	let (&last, rest) = rhs.split_last().unwrap();
	if rest.is_empty() {
		lhs == last
	} else {
		has_solution_2(lhs - last, rest)
			|| (lhs % last == 0 && has_solution_2(lhs / last, rest))
			|| lhs
				.to_string()
				.strip_suffix(&last.to_string())
				.and_then(|stripped| {
					stripped.parse().ok().map(|lhs| has_solution_2(lhs, rest))
				})
				.unwrap_or_default()
	}
}

pub fn part2() -> i64 {
	INPUT
		.lines()
		.filter_map(|line| {
			let (lhs, rhs) = parse_equation(line);
			has_solution_2(lhs, &rhs).then_some(lhs)
		})
		.sum()
}
