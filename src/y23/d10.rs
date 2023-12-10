use crate::io::read_lines;

crate::test::test_part!(test1, part1, 6875);
crate::test::test_part!(test2, part2, ?);

#[derive(Debug)]
enum Pipe {
	NorthSouth,
	EastWest,
	NorthEast,
	NorthWest,
	SouthWest,
	SouthEast,
	Ground,
	Start,
}

type Coords = (usize, usize);

impl Pipe {
	fn neighbors(&self, coords: Coords) -> [Coords; 2] {
		match self {
			Pipe::NorthSouth => {
				[(coords.0 - 1, coords.1), (coords.0 + 1, coords.1)]
			}
			Pipe::EastWest => {
				[(coords.0, coords.1 + 1), (coords.0, coords.1 - 1)]
			}
			Pipe::NorthEast => {
				[(coords.0 - 1, coords.1), (coords.0, coords.1 + 1)]
			}
			Pipe::NorthWest => {
				[(coords.0 - 1, coords.1), (coords.0, coords.1 - 1)]
			}
			Pipe::SouthWest => {
				[(coords.0 + 1, coords.1), (coords.0, coords.1 - 1)]
			}
			Pipe::SouthEast => {
				[(coords.0 + 1, coords.1), (coords.0, coords.1 + 1)]
			}
			_ => panic!("can't find neighbors of pipe variant"),
		}
	}

	fn north_open(&self) -> bool {
		matches!(self, Pipe::NorthSouth | Pipe::NorthEast | Pipe::NorthWest)
	}

	fn south_open(&self) -> bool {
		matches!(self, Pipe::NorthSouth | Pipe::SouthEast | Pipe::SouthWest)
	}

	fn east_open(&self) -> bool {
		matches!(self, Pipe::NorthEast | Pipe::SouthEast | Pipe::EastWest)
	}

	fn west_open(&self) -> bool {
		matches!(self, Pipe::NorthWest | Pipe::SouthWest | Pipe::EastWest)
	}
}

impl From<char> for Pipe {
	fn from(value: char) -> Pipe {
		match value {
			'|' => Pipe::NorthSouth,
			'-' => Pipe::EastWest,
			'L' => Pipe::NorthEast,
			'J' => Pipe::NorthWest,
			'7' => Pipe::SouthWest,
			'F' => Pipe::SouthEast,
			'.' => Pipe::Ground,
			'S' => Pipe::Start,
			_ => panic!("malformed pipe"),
		}
	}
}

pub fn part1() -> usize {
	let mut pipes = read_lines("input/2023/10.txt")
		.map(|line| line.chars().map(|c| c.into()).collect::<Vec<Pipe>>())
		.collect::<Vec<_>>();
	let mut s = (0, 0);
	for (i, row) in pipes.iter().enumerate() {
		for (j, pipe) in row.iter().enumerate() {
			if let Pipe::Start = pipe {
				s = (i, j);
			}
		}
	}
	let north = s.0 > 0 && pipes[s.0 - 1][s.1].south_open();
	let south = s.0 < pipes.len() - 1 && pipes[s.0 + 1][s.1].north_open();
	let east = s.1 < pipes[s.0].len() - 1 && pipes[s.0][s.1 + 1].west_open();
	let west = s.1 > 0 && pipes[s.0][s.1 - 1].east_open();
	pipes[s.0][s.1] = match (north, south, east, west) {
		(true, true, false, false) => Pipe::NorthSouth,
		(true, false, true, false) => Pipe::NorthEast,
		(true, false, false, true) => Pipe::NorthWest,
		(false, true, true, false) => Pipe::SouthEast,
		(false, true, false, true) => Pipe::SouthWest,
		(false, false, true, true) => Pipe::EastWest,
		_ => panic!("invalid pipe network"),
	};
	// Move in an arbitrary direction, to start.
	let mut prev = s;
	let mut coords = pipes[s.0][s.1].neighbors(s)[0];
	let mut distance = 1;
	// Move in the unvisited direction until returning to the start.
	while coords != s {
		let [n1, n2] = pipes[coords.0][coords.1].neighbors(coords);
		if n1 == prev {
			prev = coords;
			coords = n2;
		} else {
			prev = coords;
			coords = n1;
		}
		distance += 1;
	}
	distance / 2
}

pub fn part2() -> usize {
	0
}
