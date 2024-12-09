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

// TODO: Refactor into a struct with an optional ID.
#[derive(Clone, Copy)]
enum Segment {
	Space { size: usize },
	File { id: usize, size: usize },
}

struct Disk {
	segments: Vec<Segment>,
	file_count: usize,
}

impl Disk {
	fn read() -> Disk {
		let mut segments = Vec::new();
		let mut file_count = 0;
		for (i, size) in read_lines("input/09.txt")
			.next()
			.unwrap()
			.bytes()
			.map(|b| (b - b'0') as usize)
			.enumerate()
		{
			let segment = if i % 2 == 0 {
				let id = i / 2;
				file_count += 1;
				Segment::File { id, size }
			} else {
				Segment::Space { size }
			};
			segments.push(segment);
		}
		Disk {
			segments,
			file_count,
		}
	}

	fn checksum(self) -> usize {
		let mut checksum = 0;
		let mut total_size = 0;
		for segment in self.segments {
			match segment {
				Segment::Space { size } => total_size += size,
				Segment::File { id, size } => {
					for i in total_size..total_size + size {
						checksum += id * i;
					}
					total_size += size;
				}
			}
		}
		checksum
	}
}

pub fn part2() -> usize {
	let mut disk = Disk::read();

	// Defrag.
	'files: for next_id in (1..disk.file_count).rev() {
		let (i, &file) = disk
			.segments
			.iter()
			.find_position(|segment| {
				if let Segment::File { id, .. } = segment {
					*id == next_id
				} else {
					false
				}
			})
			.unwrap();
		let Segment::File {
			size: file_size, ..
		} = file
		else {
			unreachable!(); // TODO: Refactor.
		};
		for j in 1..i {
			if let Segment::Space { size } = disk.segments[j] {
				if size >= file_size {
					disk.segments.remove(i);
					disk.segments.insert(i, Segment::Space { size: file_size });
					disk.segments.remove(j);
					disk.segments.insert(j, file);
					disk.segments.insert(
						j + 1,
						Segment::Space {
							size: size - file_size,
						},
					);
					continue 'files;
				}
			}
		}
	}

	disk.checksum()
}
