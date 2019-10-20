
#[macro_use]
extern crate cycle_match;

fn main() {
	let data = b"123456789";
	
	let mut num = 0usize;
	
	let mut iter = data.iter();
	while_match!((iter) -> || {
		Some(b'0') => {},
		Some(a @ b'0' ..= b'9') => {
			num *= 10;
			num += (a - b'0') as usize;
		},
		Some(a) => panic!("Unk byte: {:?}", a),
		_ => break
	});
	
	assert_eq!(num, 123456789);
}
