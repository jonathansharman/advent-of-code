use std::collections::HashMap;

aoc::test::test_part!(test1, part1, 1315285);
aoc::test::test_part!(test2, part2, 9847279);

const INPUT: &str = include_str!("input/07.txt");

#[derive(Debug)]
enum Node {
	File(usize),
	Directory(Vec<String>),
}

fn build_nodes() -> HashMap<Vec<String>, Node> {
	let mut nodes = HashMap::new();
	nodes.insert(Vec::new(), Node::Directory(Vec::new()));
	let mut wd = Vec::new();
	for line in INPUT.lines().skip(1) {
		if line == "$ ls" {
			continue;
		} else if line == "$ cd .." {
			wd.pop();
		} else if let Some(name) = line.strip_prefix("$ cd ") {
			wd.push(name.to_string());
		} else if let Some(name) = line.strip_prefix("dir ") {
			let mut key = wd.clone();
			key.push(name.to_string());
			nodes.entry(key).or_insert(Node::Directory(Vec::new()));

			if let Some(Node::Directory(children)) = nodes.get_mut(&wd) {
				children.push(name.to_string());
			}
		} else {
			let (size, name) = line.split_once(' ').unwrap();
			let mut key = wd.clone();
			key.push(name.to_string());
			let size = size.parse().unwrap();
			nodes.entry(key).or_insert(Node::File(size));

			if let Some(Node::Directory(children)) = nodes.get_mut(&wd) {
				children.push(name.to_string());
			}
		}
	}
	nodes
}

fn sum_no_more_than_100_000(
	nodes: &HashMap<Vec<String>, Node>,
	cache: &mut HashMap<Vec<String>, usize>,
	key: Vec<String>,
) -> usize {
	let mut sum = 0;
	if let Some(Node::Directory(children)) = nodes.get(&key) {
		for child in children {
			let mut child_key = key.clone();
			child_key.push(child.clone());
			sum += sum_no_more_than_100_000(nodes, cache, child_key);
		}
		let size = get_size(nodes, cache, key);
		if size <= 100_000 {
			sum += size;
		}
	}
	sum
}

fn get_size(
	nodes: &HashMap<Vec<String>, Node>,
	cache: &mut HashMap<Vec<String>, usize>,
	key: Vec<String>,
) -> usize {
	if let Some(&size) = cache.get(&key) {
		return size;
	}
	match nodes.get(&key).unwrap() {
		Node::File(size) => {
			cache.insert(key, *size);
			*size
		}
		Node::Directory(children) => {
			let mut size = 0;
			for child in children {
				let mut child_key = key.clone();
				child_key.push(child.clone());
				size += get_size(nodes, cache, child_key);
			}
			cache.insert(key, size);
			size
		}
	}
}

pub fn part1() -> usize {
	let nodes = build_nodes();
	let mut cache = HashMap::new();
	sum_no_more_than_100_000(&nodes, &mut cache, Vec::new())
}

pub fn part2() -> usize {
	let nodes = build_nodes();
	let mut cache = HashMap::new();
	let used = get_size(&nodes, &mut cache, Vec::new());
	const TOTAL_SIZE: usize = 70_000_000;
	const REQUIRED: usize = 30_000_000;
	let min_delete = used + REQUIRED - TOTAL_SIZE;
	cache
		.iter()
		.filter_map(|(path, &size)| {
			if let Node::Directory(_) = nodes.get(path).unwrap() {
				if size >= min_delete {
					Some(size)
				} else {
					None
				}
			} else {
				None
			}
		})
		.min()
		.unwrap()
}
