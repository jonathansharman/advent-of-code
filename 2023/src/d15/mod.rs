aoc::test::test_part!(test1, part1, 512950);
aoc::test::test_part!(test2, part2, 247153);

const INPUT: &str = include_str!("input.txt");

pub fn part1() -> usize {
	INPUT.lines().next().unwrap().split(',').map(hash).sum()
}

fn hash(label: &str) -> usize {
	label
		.bytes()
		.fold(0, |acc, b| ((acc + b as usize) * 17) % 256)
}

struct LensBox {
	lenses: Vec<(String, usize)>,
}

impl LensBox {
	fn new() -> LensBox {
		LensBox { lenses: Vec::new() }
	}

	fn insert(&mut self, label: &str, lens: usize) {
		match self
			.lenses
			.iter_mut()
			.find(|(other_label, _)| other_label == label)
		{
			Some((_, target)) => *target = lens,
			None => self.lenses.push((label.to_owned(), lens)),
		}
	}

	fn remove(&mut self, label: &str) {
		self.lenses.retain(|(other_label, _)| other_label != label);
	}
}

struct LensBoxes([LensBox; 256]);

impl LensBoxes {
	fn new() -> LensBoxes {
		LensBoxes(std::array::from_fn(|_| LensBox::new()))
	}
}

pub fn part2() -> usize {
	INPUT
		.lines()
		.next()
		.unwrap()
		.split(',')
		.fold(LensBoxes::new(), |mut acc, s| {
			let (label, rhs) = s.split_once(|c| c == '-' || c == '=').unwrap();
			let h = hash(label);
			if rhs.is_empty() {
				acc.0[h].remove(label)
			} else {
				acc.0[h].insert(label, rhs.parse().unwrap())
			}
			acc
		})
		.0
		.into_iter()
		.enumerate()
		.map(|(i, lens_box)| {
			lens_box
				.lenses
				.into_iter()
				.enumerate()
				.map(|(j, (_, lens))| (i + 1) * (j + 1) * lens)
				.sum::<usize>()
		})
		.sum()
}
