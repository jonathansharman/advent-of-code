use crate::io::read_lines;

crate::test::test_part!(test1, part1, 77318);
crate::test::test_part!(test2, part2, ?);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
	Void,
	Floor,
	Wall,
}

struct Map {
	tiles: Vec<Vec<Tile>>,
	width: i32,
	height: i32,
}

impl Map {
	fn load() -> Map {
		let mut tiles: Vec<Vec<Tile>> = read_lines("input/2022/22.txt")
			.take_while(|line| !line.is_empty())
			.map(|line| {
				line.as_bytes()
					.iter()
					.map(|b| match b {
						b'.' => Tile::Floor,
						b'#' => Tile::Wall,
						_ => Tile::Void,
					})
					.collect()
			})
			.collect();
		let width = tiles.iter().map(|row| row.len()).max().unwrap();
		for row in tiles.iter_mut() {
			row.resize(width, Tile::Void);
		}
		let (width, height) = (tiles[0].len() as i32, tiles.len() as i32);
		Map {
			tiles,
			width,
			height,
		}
	}
}

#[derive(Debug)]
enum Action {
	Walk(usize),
	Right,
	Left,
}

fn get_path() -> Vec<Action> {
	let mut path = Vec::new();
	let mut n = String::new();
	for c in read_lines("input/2022/22.txt").last().unwrap().chars() {
		match c {
			'R' => {
				if !n.is_empty() {
					path.push(Action::Walk(n.parse().unwrap()));
					n.clear();
				}
				path.push(Action::Right);
			}
			'L' => {
				if !n.is_empty() {
					path.push(Action::Walk(n.parse().unwrap()));
					n.clear();
				}
				path.push(Action::Left);
			}
			_ => {
				n.push(c);
			}
		}
	}
	if !n.is_empty() {
		path.push(Action::Walk(n.parse().unwrap()));
	}
	path
}

fn get_starting_coords(map: &Map) -> [i32; 2] {
	for (col, tile) in map.tiles.first().unwrap().iter().enumerate() {
		if *tile != Tile::Void {
			return [0, col as i32];
		}
	}
	[0; 2]
}

fn facing(dir: [i32; 2]) -> i32 {
	match dir {
		[0, 1] => 0,
		[1, 0] => 1,
		[0, -1] => 2,
		_ => 3,
	}
}

fn neighbor(map: &Map, coords: [i32; 2], dir: [i32; 2]) -> [i32; 2] {
	let mut n = coords;
	loop {
		n[0] = (n[0] + dir[0] + map.height) % map.height;
		n[1] = (n[1] + dir[1] + map.width) % map.width;
		if map.tiles[n[0] as usize][n[1] as usize] != Tile::Void {
			return n;
		}
	}
}

pub fn part1() -> i32 {
	let map = Map::load();
	let path = get_path();
	let mut coords = get_starting_coords(&map);
	let mut dir = [0, 1];
	for action in path {
		match action {
			Action::Walk(count) => {
				for _ in 0..count {
					let n = neighbor(&map, coords, dir);
					if map.tiles[n[0] as usize][n[1] as usize] != Tile::Wall {
						coords = n;
					}
				}
			}
			Action::Right => dir = [dir[1], -dir[0]],
			Action::Left => dir = [-dir[1], dir[0]],
		}
	}
	1000 * (coords[0] + 1) + 4 * (coords[1] + 1) + facing(dir)
}

pub fn part2() -> usize {
	0
}
