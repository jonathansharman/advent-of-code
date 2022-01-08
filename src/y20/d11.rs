use crate::{io::read_lines, neighbors};

crate::test::test_part!(test1, part1, ?);
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> usize {
	let mut layout = read_layout();
	while evolve(&mut layout) {}
	layout
		.into_iter()
		.map(|row| {
			row.into_iter()
				.filter(|&space| space == Space::Occupied)
				.count()
		})
		.sum()
}

pub fn part2() -> usize {
	0
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Space {
	Floor,
	Empty,
	Occupied,
}

fn read_layout() -> Vec<Vec<Space>> {
	read_lines("input/2020/11.txt")
		.map(|line| {
			line.chars()
				.map(|c| match c {
					'.' => Space::Floor,
					'L' => Space::Empty,
					'#' => Space::Occupied,
					_ => panic!("invalid character"),
				})
				.collect()
		})
		.collect()
}

fn evolve(layout: &mut Vec<Vec<Space>>) -> bool {
	let mut next_layout = Vec::with_capacity(layout.len());
	let mut changed = false;
	for (i, row) in layout.iter().enumerate() {
		next_layout.push(Vec::with_capacity(row.len()));
		for (j, space) in row.iter().enumerate() {
			let neighbors = neighbors::eight(layout.len(), row.len(), i, j)
				.into_iter()
				.filter(|(k, l)| layout[*k][*l] == Space::Occupied)
				.count();
			let next_space = match space {
				Space::Empty if neighbors == 0 => {
					changed = true;
					Space::Occupied
				}
				Space::Occupied if neighbors >= 4 => {
					changed = true;
					Space::Empty
				}
				_ => *space,
			};
			next_layout[i].push(next_space);
		}
	}
	if changed {
		*layout = next_layout;
	}
	changed
}
