/// Injects a unit test for part of an Advent of Code puzzle.
///
/// - `test_name` - The injected test name.
/// - `part` - A function returning the solution to a puzzle part.
/// - `expected` - The expected answer to the puzzle. If you don't yet know the
///   answer, use the special '?' token to simply print the possible solution
///   and pass the test.
#[macro_export]
macro_rules! test_part {
	($test_name:ident, $part:expr, $expected:expr) => {
		#[test]
		fn $test_name() {
			fn duration_string(duration: std::time::Duration) -> String {
				if duration < std::time::Duration::from_micros(1) {
					format!("{} ns", duration.as_nanos())
				} else if duration < std::time::Duration::from_millis(1) {
					format!("{} μs", duration.as_micros())
				} else if duration < std::time::Duration::from_secs(1) {
					format!("{} ms", duration.as_millis())
				} else {
					format!("{:.2} s", duration.as_secs_f32())
				}
			}

			let start = std::time::SystemTime::now();
			let answer = $part();
			let elapsed = start.elapsed().unwrap();
			assert_eq!(answer, $expected);
			println!(
				"{}::{} is {:?} ({})",
				module_path!(),
				stringify!($part),
				$part(),
				duration_string(elapsed)
			);
		}
	};
	($test_name:ident, $part:expr, ?) => {
		#[test]
		fn $test_name() {
			fn duration_string(duration: std::time::Duration) -> String {
				if duration < std::time::Duration::from_micros(1) {
					format!("{} ns", duration.as_nanos())
				} else if duration < std::time::Duration::from_millis(1) {
					format!("{} μs", duration.as_micros())
				} else if duration < std::time::Duration::from_secs(1) {
					format!("{} ms", duration.as_millis())
				} else {
					format!("{:.2} s", duration.as_secs_f32())
				}
			}

			let start = std::time::SystemTime::now();
			let answer = $part();
			let elapsed = start.elapsed().unwrap();
			println!(
				"{}::{} could be {:?} ({})",
				module_path!(),
				stringify!($part),
				$part(),
				duration_string(elapsed)
			);
		}
	};
}

pub use test_part;
