use itertools::Itertools;
use std::time::Instant;

crate::test::test_part!(test1, part1, 49917929934999);
crate::test::test_part!(test2, part2, 11911316711816);

pub fn part1() -> i64 {
	let start = Instant::now();
	let threads = (1..10)
		.map(|digit| {
			let mut input = [9; 14];
			input[0] = digit;
			std::thread::spawn(move || loop {
				if monad(input) == 0 {
					let answer = input
						.into_iter()
						.reduce(|acc, digit| 10 * acc + digit)
						.unwrap();
					println!("Found an answer: {}", answer);
					return answer;
				}
				decrement(&mut input);
				if input[0] == 0 {
					return 0;
				}
			})
		})
		.collect_vec();
	let answers = threads
		.into_iter()
		.map(|thread| thread.join().unwrap())
		.collect_vec();
	println!("Finished searching in {} s", start.elapsed().as_secs_f64());
	for (i, answer) in answers.iter().enumerate() {
		println!("From thread {}: {}", i, answer);
	}
	answers.into_iter().max().unwrap()
}

#[test]
fn benchmark_part1() {
	let iters = 10_000_000;
	let start = Instant::now();
	let threads = (1..10)
		.map(|digit| {
			let mut input = [9; 14];
			input[0] = digit;
			std::thread::spawn(move || {
				for _ in 0..iters {
					if monad(input) == 0 {
						return input
							.into_iter()
							.reduce(|acc, digit| 10 * acc + digit)
							.unwrap();
					}
					decrement(&mut input);
				}
				0
			})
		})
		.collect_vec();
	let total_iters = threads.len() as u128 * iters;
	for (t, thread) in threads.into_iter().enumerate() {
		println!("{}: {}", t, thread.join().unwrap());
	}
	let elapsed = start.elapsed();
	println!(
		"{} checks in {} ms ({} ns / check)",
		total_iters,
		elapsed.as_millis(),
		elapsed.as_nanos() / total_iters,
	);
}

// Note: I know from finding 19917929934996 in part 1 that it's an upper bound,
// so I can just search from 1* on.
pub fn part2() -> i64 {
	let start = Instant::now();
	let threads = (1..10)
		.map(|digit| {
			let mut input = [1; 14];
			input[0] = digit;
			std::thread::spawn(move || loop {
				if monad(input) == 0 {
					let answer = input
						.into_iter()
						.reduce(|acc, digit| 10 * acc + digit)
						.unwrap();
					println!("Found an answer: {}", answer);
					return answer;
				}
				increment(&mut input);
				if input[1] != digit {
					return 0;
				}
			})
		})
		.collect_vec();
	let answers = threads
		.into_iter()
		.map(|thread| thread.join().unwrap())
		.collect_vec();
	println!("Finished searching in {} s", start.elapsed().as_secs_f64());
	for (i, answer) in answers.iter().enumerate() {
		println!("From thread {}: {}", i, answer);
	}
	answers.into_iter().max().unwrap()
}

fn decrement(input: &mut [i64; 14]) {
	input[13] -= 1;
	for i in (1..input.len()).rev() {
		if input[i] == 0 {
			input[i] = 9;
			input[i - 1] -= 1;
		} else {
			break;
		}
	}
}

fn increment(input: &mut [i64; 14]) {
	input[13] += 1;
	for i in (1..input.len()).rev() {
		if input[i] == 10 {
			input[i] = 1;
			input[i - 1] += 1;
		} else {
			break;
		}
	}
}

#[allow(clippy::many_single_char_names)]
fn monad(i: [i64; 14]) -> i64 {
	let mut w = 0;
	let mut x = 0;
	let mut y = 0;
	let mut z = 0;

	w = i[0];
	x *= 0;
	x += z;
	x %= 26;
	z /= 1;
	x += 15;
	x = (x == w) as i64;
	x = (x == 0) as i64;
	y *= 0;
	y += 25;
	y *= x;
	y += 1;
	z *= y;
	y *= 0;
	y += w;
	y += 15;
	y *= x;
	z += y;
	w = i[1];
	x *= 0;
	x += z;
	x %= 26;
	z /= 1;
	x += 12;
	x = (x == w) as i64;
	x = (x == 0) as i64;
	y *= 0;
	y += 25;
	y *= x;
	y += 1;
	z *= y;
	y *= 0;
	y += w;
	y += 5;
	y *= x;
	z += y;
	w = i[2];
	x *= 0;
	x += z;
	x %= 26;
	z /= 1;
	x += 13;
	x = (x == w) as i64;
	x = (x == 0) as i64;
	y *= 0;
	y += 25;
	y *= x;
	y += 1;
	z *= y;
	y *= 0;
	y += w;
	y += 6;
	y *= x;
	z += y;
	w = i[3];
	x *= 0;
	x += z;
	x %= 26;
	z /= 26;
	x += -14;
	x = (x == w) as i64;
	x = (x == 0) as i64;
	y *= 0;
	y += 25;
	y *= x;
	y += 1;
	z *= y;
	y *= 0;
	y += w;
	y += 7;
	y *= x;
	z += y;
	w = i[4];
	x *= 0;
	x += z;
	x %= 26;
	z /= 1;
	x += 15;
	x = (x == w) as i64;
	x = (x == 0) as i64;
	y *= 0;
	y += 25;
	y *= x;
	y += 1;
	z *= y;
	y *= 0;
	y += w;
	y += 9;
	y *= x;
	z += y;
	w = i[5];
	x *= 0;
	x += z;
	x %= 26;
	z /= 26;
	x += -7;
	x = (x == w) as i64;
	x = (x == 0) as i64;
	y *= 0;
	y += 25;
	y *= x;
	y += 1;
	z *= y;
	y *= 0;
	y += w;
	y += 6;
	y *= x;
	z += y;
	w = i[6];
	x *= 0;
	x += z;
	x %= 26;
	z /= 1;
	x += 14;
	x = (x == w) as i64;
	x = (x == 0) as i64;
	y *= 0;
	y += 25;
	y *= x;
	y += 1;
	z *= y;
	y *= 0;
	y += w;
	y += 14;
	y *= x;
	z += y;
	w = i[7];
	x *= 0;
	x += z;
	x %= 26;
	z /= 1;
	x += 15;
	x = (x == w) as i64;
	x = (x == 0) as i64;
	y *= 0;
	y += 25;
	y *= x;
	y += 1;
	z *= y;
	y *= 0;
	y += w;
	y += 3;
	y *= x;
	z += y;
	w = i[8];
	x *= 0;
	x += z;
	x %= 26;
	z /= 1;
	x += 15;
	x = (x == w) as i64;
	x = (x == 0) as i64;
	y *= 0;
	y += 25;
	y *= x;
	y += 1;
	z *= y;
	y *= 0;
	y += w;
	y += 1;
	y *= x;
	z += y;
	w = i[9];
	x *= 0;
	x += z;
	x %= 26;
	z /= 26;
	x += -7;
	x = (x == w) as i64;
	x = (x == 0) as i64;
	y *= 0;
	y += 25;
	y *= x;
	y += 1;
	z *= y;
	y *= 0;
	y += w;
	y += 3;
	y *= x;
	z += y;
	w = i[10];
	x *= 0;
	x += z;
	x %= 26;
	z /= 26;
	x += -8;
	x = (x == w) as i64;
	x = (x == 0) as i64;
	y *= 0;
	y += 25;
	y *= x;
	y += 1;
	z *= y;
	y *= 0;
	y += w;
	y += 4;
	y *= x;
	z += y;
	w = i[11];
	x *= 0;
	x += z;
	x %= 26;
	z /= 26;
	x += -7;
	x = (x == w) as i64;
	x = (x == 0) as i64;
	y *= 0;
	y += 25;
	y *= x;
	y += 1;
	z *= y;
	y *= 0;
	y += w;
	y += 6;
	y *= x;
	z += y;
	w = i[12];
	x *= 0;
	x += z;
	x %= 26;
	z /= 26;
	x += -5;
	x = (x == w) as i64;
	x = (x == 0) as i64;
	y *= 0;
	y += 25;
	y *= x;
	y += 1;
	z *= y;
	y *= 0;
	y += w;
	y += 7;
	y *= x;
	z += y;
	w = i[13];
	x *= 0;
	x += z;
	x %= 26;
	z /= 26;
	x += -10;
	x = (x == w) as i64;
	x = (x == 0) as i64;
	y *= 0;
	y += 25;
	y *= x;
	y += 1;
	z *= y;
	y *= 0;
	y += w;
	y += 1;
	y *= x;
	z += y;

	z
}
