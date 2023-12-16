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

enum Side {
	Top,
	Bottom,
	Left,
	Right,
}

impl Cave {
	fn height(&self) -> usize {
		self.0.len()
	}

	fn width(&self) -> usize {
		self.0[0].len()
	}

	fn get(&self, i: usize, j: usize) -> char {
		self.0[i][j]
	}

	fn count_energized(&self, side: Side, idx: usize) -> usize {
		let mut beam_sets =
			vec![vec![BeamSet::default(); self.width()]; self.height()];
		match side {
			Side::Top => match self.get(0, idx) {
				'.' | '|' => beam_sets[0][idx].d = true,
				'-' => {
					beam_sets[0][idx].l = true;
					beam_sets[0][idx].r = true;
				}
				'\\' => beam_sets[0][idx].r = true,
				'/' => beam_sets[0][idx].l = true,
				_ => {}
			},
			Side::Bottom => match self.get(self.height() - 1, idx) {
				'.' | '|' => beam_sets[self.height() - 1][idx].u = true,
				'-' => {
					beam_sets[self.height() - 1][idx].l = true;
					beam_sets[self.height() - 1][idx].r = true;
				}
				'\\' => beam_sets[self.height() - 1][idx].l = true,
				'/' => beam_sets[self.height() - 1][idx].r = true,
				_ => {}
			},
			Side::Left => match self.get(idx, 0) {
				'.' | '-' => beam_sets[idx][0].r = true,
				'|' => {
					beam_sets[idx][0].u = true;
					beam_sets[idx][0].d = true;
				}
				'\\' => beam_sets[idx][0].d = true,
				'/' => beam_sets[idx][0].u = true,
				_ => {}
			},
			Side::Right => match self.get(idx, self.width() - 1) {
				'.' | '-' => beam_sets[idx][self.width() - 1].l = true,
				'|' => {
					beam_sets[idx][self.width() - 1].u = true;
					beam_sets[idx][self.width() - 1].d = true;
				}
				'\\' => beam_sets[idx][self.width() - 1].u = true,
				'/' => beam_sets[idx][self.width() - 1].d = true,
				_ => {}
			},
		}
		loop {
			let mut unchanged = true;
			let mut set = |dest: &mut bool| {
				if !*dest {
					unchanged = false;
				}
				*dest = true;
			};
			for i in 1..self.height() {
				for j in 0..self.width() {
					if beam_sets[i - 1][j].d {
						match self.get(i, j) {
							'.' | '|' => set(&mut beam_sets[i][j].d),
							'-' => {
								set(&mut beam_sets[i][j].l);
								set(&mut beam_sets[i][j].r);
							}
							'\\' => set(&mut beam_sets[i][j].r),
							'/' => set(&mut beam_sets[i][j].l),
							_ => {}
						}
					}
				}
			}
			for i in 0..self.height() - 1 {
				for j in 0..self.width() {
					if beam_sets[i + 1][j].u {
						match self.get(i, j) {
							'.' | '|' => set(&mut beam_sets[i][j].u),
							'-' => {
								set(&mut beam_sets[i][j].l);
								set(&mut beam_sets[i][j].r);
							}
							'\\' => set(&mut beam_sets[i][j].l),
							'/' => set(&mut beam_sets[i][j].r),
							_ => {}
						}
					}
				}
			}
			for i in 0..self.height() {
				for j in 1..self.width() {
					if beam_sets[i][j - 1].r {
						match self.get(i, j) {
							'.' | '-' => set(&mut beam_sets[i][j].r),
							'|' => {
								set(&mut beam_sets[i][j].u);
								set(&mut beam_sets[i][j].d);
							}
							'\\' => set(&mut beam_sets[i][j].d),
							'/' => set(&mut beam_sets[i][j].u),
							_ => {}
						}
					}
				}
			}
			for i in 0..self.height() {
				for j in 0..self.width() - 1 {
					if beam_sets[i][j + 1].l {
						match self.get(i, j) {
							'.' | '-' => set(&mut beam_sets[i][j].l),
							'|' => {
								set(&mut beam_sets[i][j].u);
								set(&mut beam_sets[i][j].d);
							}
							'\\' => set(&mut beam_sets[i][j].u),
							'/' => set(&mut beam_sets[i][j].d),
							_ => {}
						}
					}
				}
			}
			if unchanged {
				return beam_sets
					.into_iter()
					.map(|row| {
						row.into_iter().filter(BeamSet::energized).count()
					})
					.sum();
			}
		}
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
	read_cave().count_energized(Side::Left, 0)
}

pub fn part2() -> usize {
	let cave = read_cave();
	let (n, m) = (cave.height(), cave.width());
	(0..n)
		.map(|i| {
			cave.count_energized(Side::Left, i)
				.max(cave.count_energized(Side::Right, i))
		})
		.max()
		.unwrap()
		.max(
			(0..m)
				.map(|j| {
					cave.count_energized(Side::Top, j)
						.max(cave.count_energized(Side::Bottom, j))
				})
				.max()
				.unwrap(),
		)
}
