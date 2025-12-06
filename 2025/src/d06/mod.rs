use aoc::{grid::Grid, input, input::ParseGrid};

aoc::test::test_part!(test1, part1, 5784380717354);
aoc::test::test_part!(test2, part2, 7996218225744);

fn op(s: char) -> fn(usize, usize) -> usize {
	if s == '*' { |a, b| a * b } else { |a, b| a + b }
}

pub fn part1() -> usize {
	input!()
		.lines()
		.map(|line| line.split_whitespace().collect())
		.collect::<Grid<&'static str>>()
		.cols()
		.map(|column| {
			let column = column.copied().collect::<Vec<&'static str>>();
			column[..column.len() - 1]
				.iter()
				.map(|n| n.parse().unwrap())
				.reduce(op(column[column.len() - 1].chars().next().unwrap()))
				.unwrap()
		})
		.sum()
}

pub fn part2() -> usize {
	let mut total = 0;
	let mut operands = Vec::new();
	for column in input!().parse_grid(|c| c).cols().rev() {
		let mut n = 0;
		for &c in column {
			match c {
				' ' => {
					if n != 0 {
						operands.push(n);
						n = 0;
					}
				}
				'+' | '*' => {
					if n != 0 {
						operands.push(n);
						n = 0;
					}
					total += operands.iter().copied().reduce(op(c)).unwrap();
					operands.clear();
				}
				_ => n = n * 10 + c.to_digit(10).unwrap() as usize,
			}
		}
	}
	total
}
