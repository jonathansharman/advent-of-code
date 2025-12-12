use aoc::{
	grid::{EAST, Grid, Point, SOUTH, SOUTHEAST, Vector},
	input,
};

aoc::test::test_part!(test1, part1, 2496);
aoc::test::test_part!(test2, part2, 1967);

fn xmas(word_search: &Grid<char>, p: Point, v: Vector) -> bool {
	word_search.get(p).map(|c| *c == 'X').unwrap_or(false)
		&& word_search.get(p + v).map(|c| *c == 'M').unwrap_or(false)
		&& word_search
			.get(p + 2 * v)
			.map(|c| *c == 'A')
			.unwrap_or(false)
		&& word_search
			.get(p + 3 * v)
			.map(|c| *c == 'S')
			.unwrap_or(false)
}

pub fn part1() -> usize {
	let word_search: Grid<char> = input!().lines().map(str::chars).collect();
	let mut sum = 0;
	for row in 0..word_search.row_count() {
		for col in 0..word_search.col_count() {
			for drow in -1..=1 {
				for dcol in -1..=1 {
					if drow == 0 && dcol == 0 {
						continue;
					}
					if xmas(
						&word_search,
						Point::new(row, col),
						Vector::new(drow, dcol),
					) {
						sum += 1;
					}
				}
			}
		}
	}
	sum
}

fn x_mas(word_search: &Grid<char>, p: Point) -> bool {
	matches!(
		(
			word_search[p],
			word_search[p + SOUTHEAST],
			word_search[p + 2 * SOUTHEAST]
		),
		('M', 'A', 'S') | ('S', 'A', 'M')
	) && matches!(
		(word_search[p + 2 * SOUTH], word_search[p + 2 * EAST]),
		('M', 'S') | ('S', 'M')
	)
}

pub fn part2() -> usize {
	let word_search: Grid<char> = input!().lines().map(str::chars).collect();
	let mut sum = 0;
	for row in 0..word_search.row_count() - 2 {
		for col in 0..word_search.col_count() - 2 {
			if x_mas(&word_search, Point::new(row, col)) {
				sum += 1;
			}
		}
	}
	sum
}
