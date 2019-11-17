# Cycle_match

Convenient macros for combining cycles (for, while, loop) with a match.

[![Build Status](https://travis-ci.org/clucompany/cycle_match.svg?branch=master)](https://travis-ci.org/clucompany/cycle_match)
[![Mit/Apache licensed](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](./LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/cycle_match)](https://crates.io/crates/cycle_match)
[![Documentation](https://docs.rs/cycle_match/badge.svg)](https://docs.rs/cycle_match)

Purpose: To read lines from a file ignoring comments and special characters using macros (for_match, while_match).


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
				Some(b'#') => while_match!((iter) -> || {
					Some(b'\n') => continue 'read,
					Some(_a) => {},
					_ => break 'read,
				}),
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

## Use 1 (while_match)

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
		Some(a @ b'1' ..= b'9') => {
			num *= 10;
			num += (a - b'0') as usize;
		},
		Some(a) => panic!("Unk byte: {:?}", a),
		_ => break
	});
	
	assert_eq!(num, 123456789);
}
```

## Use 2 (for_match)

Purpose: Convert characters to a digital sequence using a macro for_match.

```rust
#[macro_use]
extern crate cycle_match;

fn main() {
	let data = "123456789";
	
	let mut num = 0;	
	for_match!((data.as_bytes().into_iter()) -> || {
		Some(b'0') => {},
		Some(a @ b'1' ..= b'9') => {
			num *= 10;
			num += (a - b'0') as usize;
		},
		Some(a) => panic!("Unk byte: {:?}", a),
		_ => break num,
	});
	
	println!("{}", num);
}
```

// See the "for_match" example for a more beautiful version.

## Use 3 (loop_match)

Purpose: Count the sum of all bytes of a string using the loop_match macro.

```rust
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
```


# Entrance arguments

1. loop_match

```
	loop_match!(@'begin (num, 0 ...) -> |num_add ...| {
```

(Optional) ```@'begin```	 Labels this loop.

(Necessarily) ```num```		 1 required argument for match.

(Optional) ```0```		 Any possible value that will be available only in the body of this macro, described with a comma. This value will be associated with the name you write in | |. // (let mut num_add = 0; ...)

2. while_match

```
	
```


## What can be written in the body of macros?

1. The same thing that you write in the body of the `match` language construct.

```rust
#[macro_use]
extern crate cycle_match;

fn main() {
	let data = b"123456789";
	
	let mut num = 0usize;
	
	let mut iter = data.iter();
	while_match!((iter) -> || {
		Some(b'0') => {},
		Some(a @ b'1' ..= b'9') => {
			num *= 10;
			num += (a - b'0') as usize;
		},
		Some(a) => panic!("Unk byte: {:?}", a),
		_ => break
	});
	
	assert_eq!(num, 123456789);
}
```

2. Combined language construct `match` with remote blocks of executable code.

/// !Use only for needs that you especially need, as the macro value is lost.

The standard macro body usually consists only of the internal parts of the `match` language construct, but it is possible to add code that executes before and after match. To do this, you need to move the executable code and the `match` code to the necessary blocks.


```rust
#[macro_use]
extern crate cycle_match;

fn main() {
	let mut num = 1;
	loop_match!(@'begin (num, 0) -> |num_add| {
		#[insert] { // Possible executable code before the `match` language construct
			// Any possible executable code
		},
		#[begin] { // The body of the language construct `match`
			0 ..= 255	=> num_add += 2,
			255 ..= 655 => num_add += 3,
			655 ..= 955 => num_add += 4,
			
			_		=> break,
		},
		#[insert] { // Possible executable code after the `match` language construct
			num *= num_add;
			// Any possible executable code
		}
	});
	assert_eq!(num, 4224);
}
```
