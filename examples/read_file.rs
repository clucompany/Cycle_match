
#[macro_use]
extern crate cycle_match;

use std::io::Read;

fn main() -> Result<(), std:: io::Error> {
	
	let mut read_buffer = [0u8; 128];
	let mut buffer = Vec::with_capacity(130);
	let mut file = std::fs::File::open("./read.txt")?;
	
	
	while_match!((file.read(&mut read_buffer)) -> |_| {
		Ok(0) => break,
		Ok(len) => {
			let real_array = &read_buffer[..len];
			
			for_match!(@'read (real_array.into_iter()) -> |iter, _| {
				Some(13u8) => continue,
				Some(b'\n') => {
					if buffer.len() > 0 {
						println!("#line: {}", unsafe { std::str::from_utf8_unchecked(&buffer) });
						buffer.clear();
					}
				},
				Some(b'#') => {
					while_match!((iter) -> |_| {
						Some(b'\n') => continue 'read,
						Some(_a) => {},
						_ => break 'read,
					});
				},
				Some(a) => buffer.push(*a),
				_ => break,
			});
		},
		
		Err(e) => return Err(e),
	});
	if buffer.len() > 0 {
		println!("#line: {}", unsafe { std::str::from_utf8_unchecked(&buffer) });
		buffer.clear();
	}
	
	Ok(())
}
