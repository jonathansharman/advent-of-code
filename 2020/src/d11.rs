use aoc::{
	grid::{Grid, Point},
	io::read_lines,
};

aoc::test::test_part!(test1, part1, 2273);
aoc::test::test_part!(test2, part2, 2064);

pub fn part1() -> usize {
	let mut layout = read_layout();
	while evolve(&mut layout) {}
	layout
		.into_tiles()
		.into_iter()
		.filter(|&space| space == Space::Occupied)
		.count()
}

pub fn part2() -> usize {
	let mut layout = read_layout();
	let los_neighbors = layout
		.rows()
		.enumerate()
		.map(|(i, row)| {
			(0..row.len())
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
						line_of_sight(&layout, (i, j).into(), direction)
					})
					.collect()
				})
				.collect()
		})
		.collect();
	while evolve2(&mut layout, &los_neighbors) {}
	layout
		.tiles()
		.iter()
		.filter(|&&space| space == Space::Occupied)
		.count()
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

fn read_layout() -> Grid<Space> {
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

fn evolve(layout: &mut Grid<Space>) -> bool {
	let mut changed = false;
	let next_layout = layout
		.rows()
		.enumerate()
		.map(|(i, row)| {
			row.enumerate()
				.map(|(j, space)| {
					let coords = (i, j).into();
					let neighbors = layout
						.eight_neighbors(coords)
						.filter(|&(neighbor_coords, _)| {
							layout[neighbor_coords] == Space::Occupied
						})
						.count();
					match space {
						Space::Empty if neighbors == 0 => {
							changed = true;
							Space::Occupied
						}
						Space::Occupied if neighbors >= 4 => {
							changed = true;
							Space::Empty
						}
						_ => *space,
					}
				})
				.collect()
		})
		.collect();
	if changed {
		*layout = next_layout;
	}
	changed
}

fn evolve2(layout: &mut Grid<Space>, los_neighbors: &Grid<Vec<Point>>) -> bool {
	let mut changed = false;
	let next_layout = layout
		.rows()
		.enumerate()
		.map(|(i, row)| {
			row.enumerate()
				.map(|(j, space)| {
					let coords = (i, j).into();
					let neighbors = los_neighbors[coords]
						.iter()
						.filter(|&&neighbor| {
							layout[neighbor] == Space::Occupied
						})
						.count();
					match space {
						Space::Empty if neighbors == 0 => {
							changed = true;
							Space::Occupied
						}
						Space::Occupied if neighbors >= 5 => {
							changed = true;
							Space::Empty
						}
						_ => *space,
					}
				})
				.collect()
		})
		.collect();
	if changed {
		*layout = next_layout;
	}
	changed
}

fn line_of_sight(
	layout: &Grid<Space>,
	start: Point,
	direction: Dir,
) -> Option<Point> {
	let mut coords = start;
	while let Some(space) = layout.get(coords) {
		if coords != start {
			if let Space::Empty | Space::Occupied = space {
				return Some(coords);
			}
		}
		match direction {
			Dir::Up | Dir::UpRight | Dir::UpLeft => coords.row -= 1,
			Dir::Down | Dir::DownRight | Dir::DownLeft => coords.row += 1,
			_ => {}
		}
		match direction {
			Dir::Right | Dir::UpRight | Dir::DownRight => coords.col += 1,
			Dir::Left | Dir::UpLeft | Dir::DownLeft => coords.col -= 1,
			_ => {}
		}
	}
	None
}

#[allow(unused)]
fn print_layout(layout: &Grid<Space>) {
	for row in layout.rows() {
		for space in row {
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
