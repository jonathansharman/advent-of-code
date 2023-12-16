use crate::io::read_lines;

crate::test::test_part!(test1, part1, 7870);
crate::test::test_part!(test2, part2, ?);

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

pub fn part1() -> usize {
	let cave = read_lines("input/2023/16.txt")
		.map(|line| line.chars().collect::<Vec<_>>())
		.collect::<Vec<_>>();
	let (n, m) = (cave.len(), cave[0].len());
	let mut beam_sets = vec![vec![BeamSet::default(); m]; n];
	match cave[0][0] {
		'.' | '-' => beam_sets[0][0].r = true,
		'|' => {
			beam_sets[0][0].u = true;
			beam_sets[0][0].d = true;
		}
		'\\' => beam_sets[0][0].d = true,
		'/' => beam_sets[0][0].u = true,
		_ => {}
	};
	loop {
		let mut unchanged = true;
		let mut set = |dest: &mut bool| {
			if !*dest {
				unchanged = false;
			}
			*dest = true;
		};
		for i in 1..n {
			for j in 0..m {
				if beam_sets[i - 1][j].d {
					match cave[i][j] {
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
		for i in 0..n - 1 {
			for j in 0..m {
				if beam_sets[i + 1][j].u {
					match cave[i][j] {
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
		for i in 0..n {
			for j in 1..m {
				if beam_sets[i][j - 1].r {
					match cave[i][j] {
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
		for i in 0..n {
			for j in 0..m - 1 {
				if beam_sets[i][j + 1].l {
					match cave[i][j] {
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
			for line in beam_sets.iter() {
				for beam_set in line.iter() {
					print!("{}", if beam_set.energized() { '#' } else { '.' });
				}
				println!();
			}
			return beam_sets
				.into_iter()
				.map(|row| row.into_iter().filter(BeamSet::energized).count())
				.sum();
		}
	}
}

pub fn part2() -> usize {
	0
}
