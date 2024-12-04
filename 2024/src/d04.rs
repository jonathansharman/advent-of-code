use aoc::io::read_lines;

aoc::test::test_part!(test1, part1, ?);
aoc::test::test_part!(test2, part2, ?);

fn xmas(
	word_search: &[Vec<char>],
	row: isize,
	col: isize,
	drow: isize,
	dcol: isize,
) -> bool {
	if row + drow < 0
		|| row + drow >= word_search.len() as isize
		|| col + dcol < 0
		|| col + dcol >= word_search[0].len() as isize
	{
		return false;
	}
	let x = word_search
		.get(row as usize)
		.and_then(|line| line.get(col as usize))
		.map(|c| *c == 'X')
		.unwrap_or(false);
	let m = word_search
		.get((row + drow) as usize)
		.and_then(|line| line.get((col + dcol) as usize))
		.map(|c| *c == 'M')
		.unwrap_or(false);
	let a = word_search
		.get((row + 2 * drow) as usize)
		.and_then(|line| line.get((col + 2 * dcol) as usize))
		.map(|c| *c == 'A')
		.unwrap_or(false);
	let s = word_search
		.get((row + 3 * drow) as usize)
		.and_then(|line| line.get((col + 3 * dcol) as usize))
		.map(|c| *c == 'S')
		.unwrap_or(false);
	x && m && a && s
}

pub fn part1() -> usize {
	let word_search: Vec<Vec<char>> = read_lines("input/04.txt")
		.map(|line| line.chars().collect())
		.collect();
	let mut sum = 0;
	for row in 0..word_search.len() {
		for col in 0..word_search[row].len() {
			for drow in -1..=1 {
				for dcol in -1..=1 {
					if drow == 0 && dcol == 0 {
						continue;
					}
					if xmas(
						&word_search,
						row as isize,
						col as isize,
						drow,
						dcol,
					) {
						sum += 1;
					}
				}
			}
		}
	}
	sum
}

pub fn part2() -> usize {
	0
}
