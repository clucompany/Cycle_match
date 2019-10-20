
#[macro_use]
extern crate cycle_match;

fn main() {
	let mut num = 1;
	loop_match!(@'begin (num, 0) -> |num_add| {
		#[begin] {
			0 ..= 255	=> num_add += 2,
			255 ..= 655 => num_add += 3,
			655 ..= 955 => num_add += 4,
			
			_		=> break,
		},
		#[insert] {
			num *= num_add;
		}
	});
	assert_eq!(num, 4224);
}
