use crate::io::read_lines;

crate::test::test_part!(test1, part1, ?);
crate::test::test_part!(test2, part2, ?);

enum Tile {
	Floor,
	Cube,
	RoundedRock,
}

pub fn part1() -> usize {
	let tiles = read_lines("input/2023/14.txt")
		.map(|line| {
			line.chars()
				.map(|c| match c {
					'O' => Tile::RoundedRock,
					'#' => Tile::Cube,
					_ => Tile::Floor,
				})
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();
	let (n, m) = (tiles.len(), tiles[0].len());
	let mut load = 0;
	for j in 0..m {
		let mut pile = 0;
		for i in (0..n).rev() {
			match tiles[i][j] {
				Tile::Floor => {}
				Tile::Cube => {
					load +=
						(i + 1..i + 1 + pile).map(|ii| n - ii).sum::<usize>();
					pile = 0;
				}
				Tile::RoundedRock => pile += 1,
			}
		}
		load += (0..pile).map(|i| n - i).sum::<usize>();
	}
	load
}

pub fn part2() -> usize {
	0
}
