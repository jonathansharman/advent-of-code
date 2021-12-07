pub trait Solution {
	fn year(&self) -> u32;

	fn day(&self) -> u32;

	fn part1(&self) -> i64;

	fn part2(&self) -> i64;
}
