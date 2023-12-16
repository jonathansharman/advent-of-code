use crate::io::read_lines;

crate::test::test_part!(test1, part1, 9684228);
crate::test::test_part!(test2, part2, 483844716556);

pub fn part1() -> usize {
	solve(1)
}

pub fn part2() -> usize {
	solve(999_999)
}

fn solve(expansion: usize) -> usize {
	let map = read_lines("input/2023/11.txt")
		.map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>())
		.collect::<Vec<_>>();
	let galaxies = map
		.iter()
		.enumerate()
		.flat_map(|(i, row)| {
			row.iter()
				.enumerate()
				.filter_map(|(j, tile)| tile.then_some((i, j)))
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();
	let empty_rows = map
		.iter()
		.enumerate()
		.filter_map(|(i, row)| row.iter().all(|tile| !tile).then_some(i))
		.collect::<Vec<_>>();
	let empty_cols = (0..map[0].len())
		.filter(|&j| map.iter().all(|row| !row[j]))
		.collect::<Vec<_>>();
	let mut sum = 0;
	for (i, g1) in galaxies.iter().enumerate() {
		for g2 in galaxies.iter().skip(i) {
			// Add the basic Manhattan distance.
			sum += g1.0.abs_diff(g2.0) + g1.1.abs_diff(g2.1);
			// Add distance to account for row expansion.
			for empty_row in empty_rows.iter() {
				if (g1.0.min(g2.0)..g1.0.max(g2.0)).contains(empty_row) {
					sum += expansion;
				}
			}
			// Add distance to account for column expansion.
			for empty_col in empty_cols.iter() {
				if (g1.1.min(g2.1)..g1.1.max(g2.1)).contains(empty_col) {
					sum += expansion;
				}
			}
		}
	}
	sum
}
