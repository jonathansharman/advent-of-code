use aoc::io::read_lines;
use itertools::Itertools;

aoc::test::test_part!(test1, part1, "4,1,5,3,1,5,3,5,7");
aoc::test::test_part!(test2, part2, ?);

fn read() -> (Computer, Vec<u8>) {
	let mut lines = read_lines("input/17.txt");
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
	a: u32,
	b: u32,
	c: u32,
	out: Vec<u32>,
}

impl Computer {
	fn combo(&self, operand: u8) -> u32 {
		match operand {
			0..=3 => operand as u32,
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
				ADV => self.a /= 2u32.pow(self.combo(operand)),
				BXL => self.b ^= operand as u32,
				BST => self.b = self.combo(operand) % 8,
				JNZ => {
					if self.a != 0 {
						self.pc = operand as usize;
						continue;
					}
				}
				BXC => self.b ^= self.c,
				OUT => self.out.push(self.combo(operand) % 8),
				BDV => self.b = self.a / 2u32.pow(self.combo(operand)),
				CDV => self.c = self.a / 2u32.pow(self.combo(operand)),
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

pub fn part2() -> usize {
	0
}
