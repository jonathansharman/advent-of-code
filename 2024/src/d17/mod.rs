use std::collections::HashSet;

use aoc::input;
use itertools::Itertools;

aoc::test::test_part!(test1, part1, "4,1,5,3,1,5,3,5,7");
aoc::test::test_part!(test2, part2, 164542125272765);

fn read() -> (Computer, Vec<u8>) {
	let mut lines = input!().lines();
	let a = lines
		.next()
		.unwrap()
		.strip_prefix("Register A: ")
		.unwrap()
		.parse()
		.unwrap();
	let program = lines
		.nth(3)
		.unwrap()
		.strip_prefix("Program: ")
		.unwrap()
		.split(',')
		.map(|n| n.parse().unwrap())
		.collect();
	(
		Computer {
			pc: 0,
			a,
			b: 0,
			c: 0,
			out: Vec::new(),
		},
		program,
	)
}

const ADV: u8 = 0;
const BXL: u8 = 1;
const BST: u8 = 2;
const JNZ: u8 = 3;
const BXC: u8 = 4;
const OUT: u8 = 5;
const BDV: u8 = 6;
const CDV: u8 = 7;

struct Computer {
	pc: usize,
	a: u64,
	b: u64,
	c: u64,
	out: Vec<u8>,
}

impl Computer {
	fn combo(&self, operand: u8) -> u64 {
		match operand {
			0..=3 => operand as u64,
			4 => self.a,
			5 => self.b,
			6 => self.c,
			_ => panic!("unsupported operand"),
		}
	}

	fn exec(&mut self, program: &[u8]) {
		while let [opcode, operand] =
			program[self.pc..program.len().min(self.pc + 2)]
		{
			match opcode {
				ADV => self.a >>= self.combo(operand),
				BXL => self.b ^= operand as u64,
				BST => self.b = self.combo(operand) % 8,
				JNZ => {
					if self.a != 0 {
						self.pc = operand as usize;
						continue;
					}
				}
				BXC => self.b ^= self.c,
				OUT => self.out.push((self.combo(operand) % 8) as u8),
				BDV => self.b = self.a >> self.combo(operand),
				CDV => self.c = self.a >> self.combo(operand),
				_ => unreachable!(),
			}
			self.pc += 2;
		}
	}
}

pub fn part1() -> String {
	let (mut computer, program) = read();
	computer.exec(&program);
	computer.out.into_iter().map(|n| n.to_string()).join(",")
}

// Note that this solution is based on analysis specific to my puzzle input.
fn a_candidates(a: u64, out: &[u8]) -> HashSet<u64> {
	let (&last, rest) = out.split_last().unwrap();
	let mut results = HashSet::new();
	for word1 in 0..8 {
		let new_a = a ^ word1;
		let word2 = (new_a >> (word1 ^ 1)) % 8;
		let guess = word1 ^ 4 ^ word2;
		if guess == last as u64 {
			if rest.is_empty() {
				results.insert(new_a);
			} else {
				results.extend(a_candidates(new_a << 3, rest))
			}
		}
	}
	results
}

pub fn part2() -> u64 {
	let (_, program) = read();
	a_candidates(0, &program).into_iter().min().unwrap()
}

#[allow(unused)]
fn print_program(program: &[u8]) {
	for (i, (&opcode, &operand)) in program.iter().tuples().enumerate() {
		let combo = || match operand {
			0..=3 => operand.to_string(),
			4 => "A".to_string(),
			5 => "B".to_string(),
			6 => "C".to_string(),
			_ => "!!!".to_string(),
		};
		let literal = || operand.to_string();

		let (opcode, operand) = match opcode {
			ADV => ("adv", combo()),
			BXL => ("bxl", literal()),
			BST => ("bst", combo()),
			JNZ => ("jnz", literal()),
			BXC => ("bxc", "_".to_string()),
			OUT => ("out", combo()),
			BDV => ("bdv", combo()),
			CDV => ("cdv", combo()),
			_ => unreachable!(),
		};
		println!("{i}: {opcode} {operand}");
	}
}
