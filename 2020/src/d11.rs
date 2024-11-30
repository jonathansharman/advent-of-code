use aoc::{io::read_lines, neighbors};

aoc::test::test_part!(test1, part1, 2273);
aoc::test::test_part!(test2, part2, 2064);

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
	let mut layout = read_layout();
	let los_neighbors = (0..layout.len())
		.map(|i| {
			(0..layout[0].len())
				.map(|j| {
					[
						Dir::Up,
						Dir::UpRight,
						Dir::Right,
						Dir::DownRight,
						Dir::Down,
						Dir::DownLeft,
						Dir::Left,
						Dir::UpLeft,
					]
					.into_iter()
					.filter_map(|direction| {
						line_of_sight(&layout, (i, j), direction)
					})
					.collect::<Vec<_>>()
				})
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();
	while evolve2(&mut layout, &los_neighbors) {}
	layout
		.into_iter()
		.map(|row| {
			row.into_iter()
				.filter(|&space| space == Space::Occupied)
				.count()
		})
		.sum()
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Space {
	Floor,
	Empty,
	Occupied,
}

enum Dir {
	Up,
	UpRight,
	Right,
	DownRight,
	Down,
	DownLeft,
	Left,
	UpLeft,
}

fn read_layout() -> Vec<Vec<Space>> {
	read_lines("input/11.txt")
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

fn evolve2(
	layout: &mut Vec<Vec<Space>>,
	los_neighbors: &[Vec<Vec<(usize, usize)>>],
) -> bool {
	let mut next_layout = Vec::with_capacity(layout.len());
	let mut changed = false;
	for (i, row) in layout.iter().enumerate() {
		next_layout.push(Vec::with_capacity(row.len()));
		for (j, space) in row.iter().enumerate() {
			let neighbors = los_neighbors[i][j]
				.iter()
				.filter(|(k, l)| layout[*k][*l] == Space::Occupied)
				.count();
			let next_space = match space {
				Space::Empty if neighbors == 0 => {
					changed = true;
					Space::Occupied
				}
				Space::Occupied if neighbors >= 5 => {
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

fn line_of_sight(
	layout: &[Vec<Space>],
	start: (usize, usize),
	direction: Dir,
) -> Option<(usize, usize)> {
	let (mut i, mut j) = start;
	while let Some(space) = layout.get(i).and_then(|row| row.get(j)) {
		if (i, j) != start {
			if let Space::Empty | Space::Occupied = space {
				return Some((i, j));
			}
		}
		match direction {
			Dir::Up | Dir::UpRight | Dir::UpLeft => i -= 1,
			Dir::Down | Dir::DownRight | Dir::DownLeft => i += 1,
			_ => {}
		}
		match direction {
			Dir::Right | Dir::UpRight | Dir::DownRight => j += 1,
			Dir::Left | Dir::UpLeft | Dir::DownLeft => j -= 1,
			_ => {}
		}
	}
	None
}

#[allow(unused)]
fn print_layout(layout: &[Vec<Space>]) {
	for row in layout.iter() {
		for space in row.iter() {
			print!(
				"{}",
				match space {
					Space::Floor => '.',
					Space::Empty => 'L',
					Space::Occupied => '#',
				}
			);
		}
		println!();
	}
}
