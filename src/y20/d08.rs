use crate::io::read_lines;

crate::test::test_part!(test1, part1, 1949);
crate::test::test_part!(test2, part2, 2092);

pub fn part1() -> i64 {
	exec(&read_program()).err().unwrap()
}

pub fn part2() -> i64 {
	let program = read_program();
	for idx in (0..program.len()).rev() {
		let op = program[idx];
		match op.code {
			Code::Acc => continue,
			Code::Jmp => {
				let mut program = program.clone();
				program[idx] = Op {
					code: Code::Nop,
					arg: op.arg,
				};
				if let Ok(acc) = exec(&program) {
					return acc;
				}
			}
			Code::Nop => {
				let mut program = program.clone();
				program[idx] = Op {
					code: Code::Jmp,
					arg: op.arg,
				};
				if let Ok(acc) = exec(&program) {
					return acc;
				}
			}
		}
	}
	unreachable!()
}

#[derive(Clone, Copy)]
enum Code {
	Acc,
	Jmp,
	Nop,
}

#[derive(Clone, Copy)]
struct Op {
	code: Code,
	arg: i64,
}

fn read_program() -> Vec<Op> {
	read_lines("input/2020/08.txt")
		.map(|line| {
			let arg = line[4..].parse().unwrap();
			let code = match &line[..3] {
				"acc" => Code::Acc,
				"jmp" => Code::Jmp,
				"nop" => Code::Nop,
				_ => panic!("bad op"),
			};
			Op { code, arg }
		})
		.collect()
}

fn exec(program: &[Op]) -> Result<i64, i64> {
	let mut visited = vec![false; program.len()];
	let mut acc = 0;
	let mut pc = 0;
	loop {
		if pc == program.len() {
			return Ok(acc);
		}
		if visited[pc] {
			return Err(acc);
		}
		visited[pc] = true;
		let op = program[pc];
		match op.code {
			Code::Acc => {
				acc += op.arg;
				pc += 1;
			}
			Code::Jmp => {
				pc = (pc as i64 + op.arg) as usize;
			}
			Code::Nop => {
				pc += 1;
			}
		}
	}
}
