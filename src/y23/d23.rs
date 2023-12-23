use crate::io::read_lines;

crate::test::test_part!(test1, part1, 2414);
crate::test::test_part!(test2, part2, ?);

enum Tile {
	Path,
	Forest,
	Up,
	Down,
	Left,
	Right,
}

impl Tile {
	fn allows_up(&self) -> bool {
		!matches!(self, Tile::Forest | Tile::Down)
	}

	fn allows_down(&self) -> bool {
		!matches!(self, Tile::Forest | Tile::Up)
	}

	fn allows_left(&self) -> bool {
		!matches!(self, Tile::Forest | Tile::Right)
	}

	fn allows_right(&self) -> bool {
		!matches!(self, Tile::Forest | Tile::Up)
	}
}

pub fn part1() -> usize {
	let tiles: Vec<Vec<Tile>> = read_lines("input/2023/23.txt")
		.map(|line| {
			line.chars()
				.map(|c| match c {
					'.' => Tile::Path,
					'#' => Tile::Forest,
					'^' => Tile::Up,
					'v' => Tile::Down,
					'<' => Tile::Left,
					'>' => Tile::Right,
					_ => panic!("invalid character"),
				})
				.collect()
		})
		.collect();
	longest_walk(&tiles, (0, 1), (1, 1), 1)
}

// This assumes corridors that all eventually lead to the goal and don't loop.
fn longest_walk(
	tiles: &[Vec<Tile>],
	(i_last, j_last): (usize, usize),
	(i, j): (usize, usize),
	steps: usize,
) -> usize {
	if i == tiles.len() - 1 && j == tiles[0].len() - 2 {
		return steps;
	}
	let mut result = 0;
	if i > 0 && i_last != i - 1 && tiles[i - 1][j].allows_up() {
		result = result.max(longest_walk(tiles, (i, j), (i - 1, j), steps + 1));
	}
	if i < tiles.len() - 1 && i_last != i + 1 && tiles[i + 1][j].allows_down() {
		result = result.max(longest_walk(tiles, (i, j), (i + 1, j), steps + 1));
	}
	if j > 0 && j_last != j - 1 && tiles[i][j - 1].allows_left() {
		result = result.max(longest_walk(tiles, (i, j), (i, j - 1), steps + 1));
	}
	if j < tiles[0].len() - 1
		&& j_last != j + 1
		&& tiles[i][j + 1].allows_right()
	{
		result = result.max(longest_walk(tiles, (i, j), (i, j + 1), steps + 1));
	}
	result
}

pub fn part2() -> usize {
	0
}
