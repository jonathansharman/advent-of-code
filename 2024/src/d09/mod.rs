use aoc::input;

aoc::test::test_part!(test1, part1, 6398608069280);
aoc::test::test_part!(test2, part2, 6427437134372);

fn read_disk() -> Vec<Option<usize>> {
	let mut disk = Vec::new();
	for (i, size) in input!()
		.lines()
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
	file_indices: Vec<usize>,
}

impl Disk {
	fn read() -> Disk {
		let mut segments = Vec::new();
		let mut file_indices = Vec::new();
		for (i, size) in include_str!("input.txt")
			.lines()
			.next()
			.unwrap()
			.bytes()
			.map(|b| (b - b'0') as usize)
			.enumerate()
		{
			let id = if i % 2 == 0 {
				file_indices.push(i);
				Some(i / 2)
			} else {
				None
			};
			segments.push(Segment { id, size });
		}
		Disk {
			segments,
			file_indices,
		}
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

	// Defrag.
	'files: while let Some(i) = disk.file_indices.pop() {
		let file = disk.segments[i];
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
				// Bump any file indices affected by the insertion.
				for index in &mut disk.file_indices {
					if *index > j {
						*index += 1;
					}
				}
				continue 'files;
			}
		}
	}

	disk.checksum()
}
