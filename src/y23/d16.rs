use crate::io::read_lines;

crate::test::test_part!(test1, part1, 7870);
crate::test::test_part!(test2, part2, 8143);

#[derive(Clone, Copy, Default)]
struct BeamSet {
	u: bool,
	d: bool,
	l: bool,
	r: bool,
}

impl BeamSet {
	fn energized(&self) -> bool {
		self.u || self.d || self.l || self.r
	}
}

struct Cave(Vec<Vec<char>>);

enum Direction {
	Up,
	Down,
	Left,
	Right,
}

struct BeamFront {
	direction: Direction,
	coords: (usize, usize),
}

impl Cave {
	fn height(&self) -> usize {
		self.0.len()
	}

	fn width(&self) -> usize {
		self.0[0].len()
	}

	fn get(&self, coords: (usize, usize)) -> char {
		self.0[coords.0][coords.1]
	}

	fn energy(&self, beam_front: BeamFront) -> usize {
		let mut beam_sets =
			vec![vec![BeamSet::default(); self.width()]; self.height()];
		let mut beam_fronts = vec![beam_front];
		while !beam_fronts.is_empty() {
			let mut next_beam_fronts = Vec::new();
			for beam_front in beam_fronts {
				let c = beam_front.coords;
				match beam_front.direction {
					Direction::Up => {
						if beam_sets[c.0][c.1].u {
							continue;
						}
						beam_sets[c.0][c.1].u = true;
						match self.get(c) {
							'.' | '|' => {
								if c.0 > 0 {
									next_beam_fronts.push(BeamFront {
										direction: Direction::Up,
										coords: (c.0 - 1, c.1),
									});
								}
							}
							'-' => {
								if c.1 > 0 {
									next_beam_fronts.push(BeamFront {
										direction: Direction::Left,
										coords: (c.0, c.1 - 1),
									});
								}
								if c.1 < self.width() - 1 {
									next_beam_fronts.push(BeamFront {
										direction: Direction::Right,
										coords: (c.0, c.1 + 1),
									});
								}
							}
							'\\' => {
								if c.1 > 0 {
									next_beam_fronts.push(BeamFront {
										direction: Direction::Left,
										coords: (c.0, c.1 - 1),
									});
								}
							}
							'/' => {
								if c.1 < self.width() - 1 {
									next_beam_fronts.push(BeamFront {
										direction: Direction::Right,
										coords: (c.0, c.1 + 1),
									});
								}
							}
							_ => {}
						}
					}
					Direction::Down => {
						if beam_sets[c.0][c.1].d {
							continue;
						}
						beam_sets[c.0][c.1].d = true;
						match self.get(c) {
							'.' | '|' => {
								if c.0 < self.height() - 1 {
									next_beam_fronts.push(BeamFront {
										direction: Direction::Down,
										coords: (c.0 + 1, c.1),
									});
								}
							}
							'-' => {
								if c.1 > 0 {
									next_beam_fronts.push(BeamFront {
										direction: Direction::Left,
										coords: (c.0, c.1 - 1),
									});
								}
								if c.1 < self.width() - 1 {
									next_beam_fronts.push(BeamFront {
										direction: Direction::Right,
										coords: (c.0, c.1 + 1),
									});
								}
							}
							'\\' => {
								if c.1 < self.width() - 1 {
									next_beam_fronts.push(BeamFront {
										direction: Direction::Right,
										coords: (c.0, c.1 + 1),
									});
								}
							}
							'/' => {
								if c.1 > 0 {
									next_beam_fronts.push(BeamFront {
										direction: Direction::Left,
										coords: (c.0, c.1 - 1),
									});
								}
							}
							_ => {}
						}
					}
					Direction::Left => {
						if beam_sets[c.0][c.1].l {
							continue;
						}
						beam_sets[c.0][c.1].l = true;
						match self.get(c) {
							'.' | '-' => {
								if c.1 > 0 {
									next_beam_fronts.push(BeamFront {
										direction: Direction::Left,
										coords: (c.0, c.1 - 1),
									});
								}
							}
							'|' => {
								if c.0 > 0 {
									next_beam_fronts.push(BeamFront {
										direction: Direction::Up,
										coords: (c.0 - 1, c.1),
									});
								}
								if c.0 < self.height() - 1 {
									next_beam_fronts.push(BeamFront {
										direction: Direction::Down,
										coords: (c.0 + 1, c.1),
									});
								}
							}
							'\\' => {
								if c.0 > 0 {
									next_beam_fronts.push(BeamFront {
										direction: Direction::Up,
										coords: (c.0 - 1, c.1),
									});
								}
							}
							'/' => {
								if c.0 < self.height() - 1 {
									next_beam_fronts.push(BeamFront {
										direction: Direction::Down,
										coords: (c.0 + 1, c.1),
									});
								}
							}
							_ => {}
						}
					}
					Direction::Right => {
						if beam_sets[c.0][c.1].r {
							continue;
						}
						beam_sets[c.0][c.1].r = true;
						match self.get(c) {
							'.' | '-' => {
								if c.1 < self.width() - 1 {
									next_beam_fronts.push(BeamFront {
										direction: Direction::Right,
										coords: (c.0, c.1 + 1),
									});
								}
							}
							'|' => {
								if c.0 > 0 {
									next_beam_fronts.push(BeamFront {
										direction: Direction::Up,
										coords: (c.0 - 1, c.1),
									});
								}
								if c.0 < self.height() - 1 {
									next_beam_fronts.push(BeamFront {
										direction: Direction::Down,
										coords: (c.0 + 1, c.1),
									});
								}
							}
							'\\' => {
								if c.0 < self.height() - 1 {
									next_beam_fronts.push(BeamFront {
										direction: Direction::Down,
										coords: (c.0 + 1, c.1),
									});
								}
							}
							'/' => {
								if c.0 > 0 {
									next_beam_fronts.push(BeamFront {
										direction: Direction::Up,
										coords: (c.0 - 1, c.1),
									});
								}
							}
							_ => {}
						}
					}
				}
			}
			beam_fronts = next_beam_fronts;
		}
		beam_sets
			.into_iter()
			.map(|row| row.into_iter().filter(BeamSet::energized).count())
			.sum()
	}
}

fn read_cave() -> Cave {
	Cave(
		read_lines("input/2023/16.txt")
			.map(|line| line.chars().collect::<Vec<_>>())
			.collect(),
	)
}

pub fn part1() -> usize {
	read_cave().energy(BeamFront {
		direction: Direction::Right,
		coords: (0, 0),
	})
}

pub fn part2() -> usize {
	let cave = read_cave();
	let ud = (0..cave.width())
		.map(|i| {
			let u = cave.energy(BeamFront {
				direction: Direction::Up,
				coords: (cave.height() - 1, i),
			});
			let d = cave.energy(BeamFront {
				direction: Direction::Down,
				coords: (0, i),
			});
			u.max(d)
		})
		.max()
		.unwrap();
	let lr = (0..cave.height())
		.map(|i| {
			let l = cave.energy(BeamFront {
				direction: Direction::Left,
				coords: (i, cave.width() - 1),
			});
			let r = cave.energy(BeamFront {
				direction: Direction::Right,
				coords: (i, 0),
			});
			l.max(r)
		})
		.max()
		.unwrap();
	ud.max(lr)
}
