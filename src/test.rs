macro_rules! test_part {
	($test_name:ident, $part:expr, $expected:expr) => {
		#[test]
		fn $test_name() {
			assert_eq!($expected, $part());
		}
	};
	($test_name:ident, $part:expr, ?) => {
		#[test]
		fn $test_name() {
			println!(
				"{}::{} could be {}",
				module_path!(),
				stringify!($part),
				$part()
			);
		}
	};
}

pub(crate) use test_part;
