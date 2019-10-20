//Copyright (c) 2019 #UlinProject Denis Kotlyarov (Денис Котляров)

//-----------------------------------------------------------------------------
//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//	   http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.
//-----------------------------------------------------------------------------

// or

//-----------------------------------------------------------------------------
//Permission is hereby granted, free of charge, to any person obtaining a copy
//of this software and associated documentation files (the "Software"), to deal
//in the Software without restriction, including without limitation the rights
//to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//copies of the Software, and to permit persons to whom the Software is
//furnished to do so, subject to the following conditions:

//The above copyright notice and this permission notice shall be included in all
//copies or substantial portions of the Software.

//THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//SOFTWARE.

// #Ulin Project 1819

/*!
Convenient macros for combining cycles (for, while) with a match.


# Full use

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


*/

#![no_std]

#[macro_use]
mod while_match;

#[macro_use]
mod for_match;

#[macro_use]
mod loop_match;

#[doc(hidden)]
#[macro_export]
macro_rules! cycle_matchbegin {
	[@$a:tt ($($t:tt)*): $(#[$name:tt] {$($tt:tt)*}),* $(,)? ] => {{
		$crate::cycle_matchbegin_decoder! {
			@$a ($($t)*):	
			
			$(
				#[$name] {$($tt)*}
			)*
		}
		
	}};
	
	[@$a:tt ($($t:tt)*): $($tt:tt)* ] => {{
		match $($t)* {
			$($tt)*
		}
	}};
}

#[doc(hidden)]
#[macro_export]
macro_rules! cycle_matchbegin_decoder {
	[@$a:tt ($($t:tt)*):	#[begin] {$($tt:tt)*} $($all_data:tt)*] => {
		$crate::cycle_matchbegin! {
			@$a ($($t)*): $($tt)*
		};
		
		$crate::cycle_matchbegin_decoder! {
			@$a ():
			$($all_data)*
		}
	};
	
	[@$a:tt ($($t:tt)*):	#[insert] {$($tt:tt)*} $($all_data:tt)*] => {
		{$($tt)*};
		
		$crate::cycle_matchbegin_decoder! {
			@$a ($($t)*):
			
			$($all_data)*
		}
	};
	
	[@$a:tt ($($t:tt)*):] => {};
	[] => {}
}

#[doc(hidden)]
#[macro_export]
macro_rules! cycle_variable_init {
	[ [$nn_e:expr] : _ $($tt:tt)*] => {
		{ $nn_e };
		
		$crate::cycle_variable_init! {
			$($tt)*
		}
	};
	[ [$nn_e:expr] : $nn:ident $($tt:tt)*] => {
		let mut $nn = $nn_e;
		
		$crate::cycle_variable_init! {
			$($tt)*
		}
	};
	() => ()
}
