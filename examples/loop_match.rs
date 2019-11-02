
#[macro_use]
extern crate cycle_match;

fn main() {
	let data = "1234567890";
	
	let mut data_n_index = 0usize;
	let mut iter = data.as_bytes().into_iter();
	loop_match!((iter.next()) -> || {
		Some(a) => data_n_index += *a as usize,
		_ => break,
	});
	
	assert_eq!(data_n_index, 525);
}
