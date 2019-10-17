//Copyright (c) 2019 #UlinProject Denis Kotlyarov (Денис Котляров)

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


*/

#![no_std]


#[macro_export]
macro_rules! while_match {
	( ($($t:tt)+) -> |let mut $ident:ident| { $($tt:tt)* } ) => {{
		let mut $ident;
		
		$crate::while_match!(
			($($t)+) -> |$ident| { $($tt)* }
		)
	}};
	
	( ($($t:tt)+) -> |$ident:ident| { $($tt:tt)* } ) => {{
		loop {
			$ident = $($t)*;
			$crate::decode_match!(($ident) $($tt)*);
		}
	}};
}

#[macro_export]
macro_rules! for_match {
	( ($($t:tt)*) -> |let mut $n:ident| { $($tt:tt)* } ) => {{
		let mut $n;
		
		$crate::for_match!(
			($($t)*) -> |$n| { $($tt)* }
		)
	}};
	
	( ($($t:tt)*) -> |$ident:ident| { $($tt:tt)* } ) => {{
		let mut iter = ($($t)*).iter();
		
		loop {
			$ident = iter.next();
			$crate::decode_match!(($ident) $($tt)*);
		}
	}};
}

#[macro_export]
macro_rules! decode_match {
	
	
	[ ($($t:tt)*) @end |$name:ident| => {$($d:tt)*}, $($tt:tt)* ] => {{
		match $($t)* {
			$($tt)*
		}
		
		{
			let $name = $($t)*;
			$($d)*
		}
	}};
	[ ($($t:tt)*) @start |$name:ident| => {$($d:tt)*}, $($tt:tt)*] => {
		{
			let $name = $($t)*;
			$($d)*
		}
		
		$crate::decode_match!(($($t)*) $($tt)*);
	};
	
	[ ($($t:tt)*) $($tt:tt)* ] => {{
		match $($t)* {
			$($tt)*
		}
	}};
}