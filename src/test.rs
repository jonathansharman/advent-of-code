/// Injects a unit test for part of an Advent of Code puzzle.
///
/// - `test_name` - The injected test name.
/// - `part` - A function returning the solution to a puzzle part.
/// - `answer` - The answer to the puzzle. If you don't yet know the answer, use
///   the special '?' token to simply print the possible solution and pass the
///   test.
#[macro_export]
macro_rules! test_part {
	($test_name:ident, $part:expr, $answer:expr) => {
		#[test]
		fn $test_name() {
			assert_eq!($answer, $part());
		}
	};
	($test_name:ident, $part:expr, ?) => {
		#[test]
		fn $test_name() {
			println!(
				"{}::{} could be {:?}",
				module_path!(),
				stringify!($part),
				$part()
			);
		}
	};
}

pub use test_part;
