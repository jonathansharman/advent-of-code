use aoc::grid::Grid;

aoc::test::test_part!(test1, part1, ?);
aoc::test::test_part!(test2, part2, ?);

const INPUT: &str = include_str!("input.txt");

struct Tile {
	id: usize,
	grid: Grid<bool>,
}

impl Tile {
	fn edges(&self) -> impl Iterator<Item = Vec<&bool>> {
		[
			self.grid.get_row(0).unwrap().collect(),
			self.grid
				.get_row(self.grid.row_count() - 1)
				.unwrap()
				.collect(),
			self.grid.get_col(0).unwrap().collect(),
			self.grid
				.get_col(self.grid.col_count() - 1)
				.unwrap()
				.collect(),
		]
		.into_iter()
	}
}

fn read_tiles() -> Vec<Tile> {
	let mut tiles = Vec::new();
	let mut lines = INPUT.lines();
	while let Some(label) = lines.next() {
		let id = label
			.strip_prefix("Tile ")
			.and_then(|s| s.strip_suffix(":"))
			.unwrap()
			.parse()
			.unwrap();
		let grid = lines
			.by_ref()
			.take_while(|line| !line.is_empty())
			.map(|line| line.chars().map(|c| c == '#').collect())
			.collect();
		tiles.push(Tile { id, grid });
	}
	tiles
}

pub fn part1() -> usize {
	let tiles = read_tiles();
	tiles
		.iter()
		.enumerate()
		.filter(|(i, first)| {
			tiles
				.iter()
				.enumerate()
				.filter(|(j, second)| {
					i != j
						&& first.edges().any(|edge1| {
							second.edges().any(|edge2| {
								edge1 == edge2
									|| Iterator::eq(
										edge1.iter().rev(),
										edge2.iter(),
									)
							})
						})
				})
				.count() == 2
		})
		.map(|(_, tile)| tile.id)
		.product()
}

pub fn part2() -> usize {
	0
}
