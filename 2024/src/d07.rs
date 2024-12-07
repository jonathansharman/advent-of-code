use aoc::io::read_lines;

aoc::test::test_part!(test1, part1, 882304362421);
aoc::test::test_part!(test2, part2, ?);

struct Equation {
	lhs: i64,
	rhs: Vec<i64>,
}

impl From<&str> for Equation {
	fn from(value: &str) -> Self {
		let (lhs, rhs) = value.split_once(": ").unwrap();
		Equation {
			lhs: lhs.parse().unwrap(),
			rhs: rhs.split(' ').map(|n| n.parse().unwrap()).collect(),
		}
	}
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
	read_lines("input/07.txt")
		.filter_map(|line| {
			let equation: Equation = line.as_str().into();
			has_solution(equation.lhs, &equation.rhs).then_some(equation.lhs)
		})
		.sum()
}

pub fn part2() -> usize {
	0
}
