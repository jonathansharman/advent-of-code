use aoc::{grid::Grid, input};
use itertools::Itertools;

aoc::test::test_part!(test1, part1, 3320);

pub fn part1() -> usize {
	let mut locks = Vec::new();
	let mut keys = Vec::new();

	let mut lines = input!().lines();
	loop {
		let grid: Grid<char> = lines.by_ref().take(7).map(str::chars).collect();
		let item: Vec<usize> = grid
			.cols()
			.map(|col| col.filter(|&&c| c == '#').count())
			.collect();
		if grid.get_row(0).unwrap().copied().eq("#####".chars()) {
			locks.push(item);
		} else {
			keys.push(item);
		}

		if lines.next().is_none() {
			break;
		}
	}

	locks
		.iter()
		.cartesian_product(keys.iter())
		.filter(|(lock, key)| {
			lock.iter()
				.zip(key.iter())
				.all(|(pin, tooth)| pin + tooth <= 7)
		})
		.count()
}
