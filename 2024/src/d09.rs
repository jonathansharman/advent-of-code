use aoc::io::read_lines;
use itertools::Itertools;

aoc::test::test_part!(test1, part1, 6398608069280);
aoc::test::test_part!(test2, part2, 6427437134372);

fn read_disk() -> Vec<Option<usize>> {
	let mut disk = Vec::new();
	for (i, size) in read_lines("input/09.txt")
		.next()
		.unwrap()
		.bytes()
		.map(|b| (b - b'0') as usize)
		.enumerate()
	{
		disk.extend(vec![(i % 2 == 0).then_some(i / 2); size]);
	}
	disk
}

fn checksum(disk: &[Option<usize>]) -> usize {
	disk.iter()
		.enumerate()
		.filter_map(|(i, id)| id.map(|id| i * id))
		.sum()
}

pub fn part1() -> usize {
	let mut disk = read_disk();

	// Compact.
	let mut left = 0;
	let mut right = disk.len() - 1;
	'outer: loop {
		while disk[left].is_some() {
			left += 1;
			if left == right {
				break 'outer;
			}
		}
		while disk[right].is_none() {
			right -= 1;
			if left == right {
				break 'outer;
			}
		}
		disk.swap(left, right);
	}

	checksum(&disk)
}

#[derive(Clone, Copy)]
struct Segment {
	id: Option<usize>,
	size: usize,
}

struct Disk {
	segments: Vec<Segment>,
}

impl Disk {
	fn read() -> Disk {
		let mut segments = Vec::new();
		for (i, size) in read_lines("input/09.txt")
			.next()
			.unwrap()
			.bytes()
			.map(|b| (b - b'0') as usize)
			.enumerate()
		{
			segments.push(Segment {
				id: (i % 2 == 0).then_some(i / 2),
				size,
			});
		}
		Disk { segments }
	}

	fn checksum(self) -> usize {
		let mut checksum = 0;
		let mut total_size = 0;
		for segment in self.segments {
			if let Some(id) = segment.id {
				// Could replace this with triangular number magic, but LLVM
				// might be doing it anyway.
				for i in total_size..total_size + segment.size {
					checksum += id * i;
				}
			}
			total_size += segment.size;
		}
		checksum
	}
}

pub fn part2() -> usize {
	let mut disk = Disk::read();
	// Both sample and real input end in a file.
	let max_id = disk.segments.last().unwrap().id.unwrap();

	// Defrag.
	'files: for next_id in (1..=max_id).rev() {
		let (i, &file) = disk
			.segments
			.iter()
			.find_position(|segment| {
				segment.id.map(|id| id == next_id).unwrap_or_default()
			})
			.unwrap();
		for j in 1..i {
			let space = disk.segments[j];
			if space.id.is_none() && space.size >= file.size {
				disk.segments[i].id = None;
				disk.segments[j] = file;
				disk.segments.insert(
					j + 1,
					Segment {
						id: None,
						size: space.size - file.size,
					},
				);
				continue 'files;
			}
		}
	}

	disk.checksum()
}
