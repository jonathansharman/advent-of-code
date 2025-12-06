use aoc::{grid::Grid, input};

aoc::test::test_part!(test1, part1, 5784380717354);
aoc::test::test_part!(test2, part2, ?);

struct Problem {
	operands: Vec<usize>,
	operation: fn(usize, usize) -> usize,
}

fn parse_problems() -> Vec<Problem> {
	let mut grid: Grid<&'static str> = input!()
		.lines()
		.map(|line| line.split_whitespace().collect())
		.collect();
	grid.transpose();
	grid.rows()
		.map(|column| {
			let column = column.copied().collect::<Vec<&'static str>>();
			Problem {
				operands: column[..column.len() - 1]
					.iter()
					.map(|n| n.parse().unwrap())
					.collect(),
				operation: if column[column.len() - 1] == "*" {
					|a, b| a * b
				} else {
					|a, b| a + b
				},
			}
		})
		.collect()
}

pub fn part1() -> usize {
	parse_problems()
		.into_iter()
		.map(|problem| {
			problem
				.operands
				.into_iter()
				.reduce(problem.operation)
				.unwrap()
		})
		.sum()
}

pub fn part2() -> usize {
	0
}
