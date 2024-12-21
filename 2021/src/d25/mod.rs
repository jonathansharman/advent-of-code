aoc::test::test_part!(test1, part1, 513);

const INPUT: &str = include_str!("input.txt");

pub fn part1() -> i64 {
	let mut region = read_region();
	let n = region.len();
	let m = region[0].len();
	let mut iters = 1;
	loop {
		let mut unchanged = true;
		// East
		let mut next_region = region.clone();
		for (i, row) in region.iter().enumerate() {
			for (j, space) in row.iter().enumerate() {
				let right = (j + 1) % m;
				if *space == b'>' && row[right] == b'.' {
					next_region[i][j] = b'.';
					next_region[i][right] = b'>';
					unchanged = false;
				}
			}
		}
		region = next_region;
		// South
		let mut next_region = region.clone();
		for (i, row) in region.iter().enumerate() {
			let down = (i + 1) % n;
			for (j, space) in row.iter().enumerate() {
				if *space == b'v' && region[down][j] == b'.' {
					next_region[i][j] = b'.';
					next_region[down][j] = b'v';
					unchanged = false;
				}
			}
		}
		if unchanged {
			break;
		}
		region = next_region;
		iters += 1;
	}
	iters
}

fn read_region() -> Vec<Vec<u8>> {
	INPUT.lines().map(|line| line.as_bytes().to_vec()).collect()
}
