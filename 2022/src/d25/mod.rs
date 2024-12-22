use std::{
	fmt::{Debug, Display},
	ops::{Add, AddAssign},
	str::FromStr,
};

use aoc::input;

aoc::test::test_part!(test1, part1, "2=2-1-010==-0-1-=--2");

/// Little-endian representation of SNAFU numbers.
struct Snafu {
	snafits: Vec<i8>,
}

#[derive(Debug)]
struct ParseSnafuError;

impl FromStr for Snafu {
	type Err = ParseSnafuError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut snafits = Vec::new();
		for c in s.chars() {
			match c {
				'=' => snafits.insert(0, -2),
				'-' => snafits.insert(0, -1),
				'0' => snafits.insert(0, 0),
				'1' => snafits.insert(0, 1),
				'2' => snafits.insert(0, 2),
				_ => return Err(ParseSnafuError),
			};
		}
		Ok(Self { snafits })
	}
}

impl Display for Snafu {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for snafit in self.snafits.iter().rev() {
			match snafit {
				-2 => write!(f, "=")?,
				-1 => write!(f, "-")?,
				0 => write!(f, "0")?,
				1 => write!(f, "1")?,
				_ => write!(f, "2")?,
			};
		}
		Ok(())
	}
}

impl AddAssign<Snafu> for Snafu {
	fn add_assign(&mut self, rhs: Snafu) {
		if self.snafits.len() < rhs.snafits.len() {
			self.snafits.resize(rhs.snafits.len(), 0);
		}
		for (i, rhs_snafit) in rhs.snafits.into_iter().enumerate() {
			self.snafits[i] += rhs_snafit;
		}
		for i in 0..self.snafits.len() - 1 {
			if self.snafits[i] < -2 {
				// Borrow.
				self.snafits[i] += 5;
				if i < self.snafits.len() - 1 {
					self.snafits[i + 1] -= 1;
				} else {
					self.snafits.push(-1);
				}
			} else if self.snafits[i] > 2 {
				// Carry.
				self.snafits[i] -= 5;
				if i < self.snafits.len() - 1 {
					self.snafits[i + 1] += 1;
				} else {
					self.snafits.push(1);
				}
			}
		}
	}
}

impl Add<Snafu> for Snafu {
	type Output = Self;

	fn add(mut self, rhs: Snafu) -> Self::Output {
		self += rhs;
		self
	}
}

impl From<Snafu> for i64 {
	fn from(snafu: Snafu) -> Self {
		let mut dec = 0;
		for snafit in snafu.snafits {
			dec = 5 * dec + snafit as i64;
		}
		dec
	}
}

pub fn part1() -> String {
	input!()
		.lines()
		.map(|snafu| snafu.parse::<Snafu>().unwrap())
		.reduce(|a, b| a + b)
		.unwrap()
		.to_string()
}
