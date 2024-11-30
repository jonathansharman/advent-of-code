use aoc::io::read_lines;

aoc::test::test_part!(test1, part1, 77318);
aoc::test::test_part!(test2, part2, 126017);

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
		let mut tiles: Vec<Vec<Tile>> = read_lines("input/22.txt")
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
	for c in read_lines("input/22.txt").last().unwrap().chars() {
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

fn map_neighbor(map: &Map, coords: [i32; 2], dir: [i32; 2]) -> [i32; 2] {
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
					let n = map_neighbor(&map, coords, dir);
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

// Assumed cube net:
//                 0        1
//            a────────b────────c
//            │        │        │
//           2│        │        │3
//            │        │        │
//            d────────┼────────e
//            │        │   4
//           5│        │6
//        7   │        │
//   d────────┼────────e
//   │        │        │
//  8│        │        │9
//   │        │        │
//   a────────┼────────c
//   │        │   10
// 11│        │12
//   │        │
//   b────────c
//       13
fn cube_neighbor(coords: [i32; 2], mut dir: [i32; 2]) -> ([i32; 2], [i32; 2]) {
	let mut n = [coords[0] + dir[0], coords[1] + dir[1]];
	match n[0] {
		-1 => {
			if n[1] < 100 {
				// 0 -> 11
				n = [100 + n[1], 0];
				dir = [0, 1];
			} else {
				// 1 -> 13
				n = [199, n[1] - 100];
			}
		}
		0..=49 => match n[1] {
			49 => {
				// 2 -> 8
				n = [149 - n[0], 0];
				dir = [0, 1];
			}
			150 => {
				// 3 -> 9
				n = [149 - n[0], 99];
				dir = [0, -1];
			}
			_ => {}
		},
		50..=99 => {
			if n[1] >= 100 && dir == [1, 0] {
				// 4 -> 6
				n = [n[1] - 50, 99];
				dir = [0, -1];
			} else if n[1] == 49 && dir == [0, -1] {
				// 5 -> 7
				n = [100, n[0] - 50];
				dir = [1, 0];
			} else if n[1] == 100 {
				// 6 -> 4
				n = [49, n[0] + 50];
				dir = [-1, 0];
			} else if n[1] < 50 {
				// 7 -> 5
				n = [50 + n[1], 50];
				dir = [0, 1];
			}
		}
		100..=149 => {
			if n[1] == -1 {
				// 8 -> 2
				n = [149 - n[0], 50];
				dir = [0, 1];
			} else if n[1] == 100 {
				// 9 -> 3
				n = [149 - n[0], 149];
				dir = [0, -1];
			}
		}
		150..=199 => {
			if n[1] >= 50 {
				if dir == [1, 0] {
					// 10 -> 12
					n = [n[1] + 100, 49];
					dir = [0, -1];
				} else {
					// 12 -> 10
					n = [149, n[0] - 100];
					dir = [-1, 0];
				}
			} else if n[1] == -1 {
				// 11 -> 0
				n = [0, n[0] - 100];
				dir = [1, 0];
			}
		}
		200 => {
			// 13 -> 1
			n = [0, n[1] + 100];
			dir = [1, 0];
		}
		_ => panic!("Out of bounds"),
	}
	(n, dir)
}

pub fn part2() -> i32 {
	let map = Map::load();
	let path = get_path();
	let mut coords = get_starting_coords(&map);
	let mut dir = [0, 1];
	for action in path {
		match action {
			Action::Walk(count) => {
				for _ in 0..count {
					let (ncoords, ndir) = cube_neighbor(coords, dir);
					if map.tiles[ncoords[0] as usize][ncoords[1] as usize]
						!= Tile::Wall
					{
						coords = ncoords;
						dir = ndir;
					}
				}
			}
			Action::Right => dir = [dir[1], -dir[0]],
			Action::Left => dir = [-dir[1], dir[0]],
		}
	}
	1000 * (coords[0] + 1) + 4 * (coords[1] + 1) + facing(dir)
}
