use crate::io::read_lines;

crate::test::test_part!(test1, part1, 759);
crate::test::test_part!(test2, part2, 45763);

pub fn part1() -> i64 {
	let mut x = 0;
	let mut y = 0;
	let mut dx = 1;
	let mut dy = 0;
	for line in read_lines("input/2020/12.txt") {
		let (direction, amount) = line.split_at(1);
		let amount = amount.parse::<i64>().unwrap();
		match direction {
			"N" => y += amount,
			"S" => y -= amount,
			"E" => x += amount,
			"W" => x -= amount,
			"F" => {
				x += dx * amount;
				y += dy * amount;
			}
			"L" => match amount {
				90 => {
					let t = -dy;
					dy = dx;
					dx = t;
				}
				270 => {
					let t = dy;
					dy = -dx;
					dx = t;
				}
				180 => {
					dx = -dx;
					dy = -dy;
				}
				_ => panic!("invalid angle"),
			},
			"R" => match amount {
				90 => {
					let t = dy;
					dy = -dx;
					dx = t;
				}
				270 => {
					let t = -dy;
					dy = dx;
					dx = t;
				}
				180 => {
					dx = -dx;
					dy = -dy;
				}
				_ => panic!("invalid angle"),
			},
			_ => panic!("invalid byte"),
		}
	}
	x.abs() + y.abs()
}

pub fn part2() -> i64 {
	let (mut x, mut y) = (0, 0);
	let (mut fx, mut fy) = (10, 1);
	for line in read_lines("input/2020/12.txt") {
		let (direction, amount) = line.split_at(1);
		let amount = amount.parse::<i64>().unwrap();
		match direction {
			"N" => fy += amount,
			"S" => fy -= amount,
			"E" => fx += amount,
			"W" => fx -= amount,
			"F" => {
				x += fx * amount;
				y += fy * amount;
			}
			"L" => match amount {
				90 => {
					let t = -fy;
					fy = fx;
					fx = t;
				}
				270 => {
					let t = fy;
					fy = -fx;
					fx = t;
				}
				180 => {
					fx = -fx;
					fy = -fy;
				}
				_ => panic!("invalid angle"),
			},
			"R" => match amount {
				90 => {
					let t = fy;
					fy = -fx;
					fx = t;
				}
				270 => {
					let t = -fy;
					fy = fx;
					fx = t;
				}
				180 => {
					fx = -fx;
					fy = -fy;
				}
				_ => panic!("invalid angle"),
			},
			_ => panic!("invalid byte"),
		}
	}
	x.abs() + y.abs()
}
