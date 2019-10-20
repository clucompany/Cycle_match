# cycle_match
Convenient macros for combining cycles (for, while, loop) with a match.

[![Build Status](https://travis-ci.org/clucompany/cycle_match.svg?branch=master)](https://travis-ci.org/clucompany/cycle_match)
[![Mit/Apache licensed](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](./LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/cycle_match)](https://crates.io/crates/cycle_match)
[![Documentation](https://docs.rs/cycle_match/badge.svg)](https://docs.rs/cycle_match)

Purpose: To read lines from a file ignoring comments and special characters using macros (for_match, while_match, loop).

```rust

#[macro_use]
extern crate cycle_match;

use std::io::Read;

fn main() -> Result<(), std:: io::Error> {
	let mut read_buffer = [0u8; 128];
	let mut buffer = Vec::with_capacity(130);
	let mut file = std::fs::File::open("./read.txt")?;
	
	while_match!((file.read(&mut read_buffer)) -> || {
		Ok(0) => break,
		Ok(len) => {
			let real_array = &read_buffer[..len];
			
			for_match!(@'read (real_array.into_iter()) -> |iter| {
				Some(13u8) => continue,
				Some(b'\n') => {
					if buffer.len() > 0 {
						println!("#line: {}", unsafe { std::str::from_utf8_unchecked(&buffer) });
						buffer.clear();
					}
				},
				Some(b'#') => {
					while_match!((iter) -> || {
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
	}
	
	Ok(())
}
```

# Use 1 (while_match)

Purpose: Convert characters to a digital sequence using a macro while_math.

```rust
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
```

