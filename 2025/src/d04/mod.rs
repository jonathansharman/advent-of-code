use std::collections::BTreeSet;

use aoc::{
	grid::{Grid, Point},
	input,
	input::ParseGrid,
};

aoc::test::test_part!(test1, part1, 1569);
aoc::test::test_part!(test2, part2, 9280);

fn get_rolls() -> Grid<bool> {
	input!().parse_grid(|c| c == '@')
}

fn is_free(rolls: &Grid<bool>, p: Point) -> bool {
	rolls.eight_neighbors(p).filter(|&(_, &n)| n).count() < 4
}

fn free_coords(rolls: &Grid<bool>) -> impl Iterator<Item = Point> {
	rolls
		.iter()
		.filter_map(move |(p, &r)| (r && is_free(rolls, p)).then_some(p))
}

pub fn part1() -> usize {
	free_coords(&get_rolls()).count()
}

pub fn part2() -> usize {
	let mut rolls = get_rolls();
	let mut queue = free_coords(&rolls).collect::<BTreeSet<_>>();
	let mut removed = 0;
	while let Some(p) = queue.pop_first() {
		if !rolls[p] || !is_free(&rolls, p) {
			continue;
		}
		rolls[p] = false;
		removed += 1;
		queue.extend(
			rolls.eight_neighbors(p).filter_map(|(p, n)| n.then_some(p)),
		);
	}
	removed
}
