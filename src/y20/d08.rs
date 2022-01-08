use crate::io::read_lines;

crate::test::test_part!(test1, part1, 1949);
crate::test::test_part!(test2, part2, ?);

pub fn part1() -> i64 {
	let program = read_program();
	let mut visited = vec![false; program.len()];
	let mut acc = 0;
	let mut pc = 0;
	loop {
		if visited[pc] {
			return acc;
		}
		visited[pc] = true;
		match program[pc] {
			Op::Acc(arg) => {
				acc += arg;
				pc += 1;
			}
			Op::Jmp(arg) => {
				pc = (pc as i64 + arg) as usize;
			}
			Op::Nop => {
				pc += 1;
			}
		}
	}
}

pub fn part2() -> usize {
	read_lines("input/2020/08.txt").count()
}

enum Op {
	Acc(i64),
	Jmp(i64),
	Nop,
}

fn read_program() -> Vec<Op> {
	read_lines("input/2020/08.txt")
		.map(|line| match &line[..3] {
			"acc" => Op::Acc(line[4..].parse().unwrap()),
			"jmp" => Op::Jmp(line[4..].parse().unwrap()),
			"nop" => Op::Nop,
			_ => panic!("bad op"),
		})
		.collect()
}
