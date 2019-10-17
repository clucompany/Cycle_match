
#[macro_use]
extern crate cycle_match;

fn main() {
	let data = b"1234561";
	
	let mut num = 0;
	
	for_match!((data) -> |let mut a| {
		@end |_a| => {
			num *= 10;
		},
		
		Some(b'0') => {},
		Some(b'1') => num += 1,
		Some(b'2') => num += 2,
		Some(b'3') => num += 3,
		Some(b'4') => num += 4,
		Some(b'5') => num += 5,
		Some(b'6') => num += 6,
		Some(b'7') => num += 7,
		Some(b'8') => num += 8,
		Some(b'9') => num += 9,
		
		Some(a) => panic!("Unk byte: {:?}", a),
		_ => break,
	});
	
	println!("{}", num);
}
