

/// [DOC IS ATTACHED!] The `For` loop, combined with matching.
/// # Full use
/// ```rust
///
///#[macro_use]
///extern crate cycle_match;
///fn main() {
///	let data = "12345678901";
///	
///	let mut a;
///	let result = for_match!(@'begin (data.as_bytes().into_iter(), a, 0usize) -> |iter, num| {
///		Some(b'0') => {},
///		Some(b'1') => num += 1,
///		Some(b'2') => num += 2,
///		Some(b'3') => num += 3,
///		Some(b'4') => num += 4,
///		Some(b'5') => num += 5,
///		Some(b'6') => num += 6,
///		Some(b'7') => num += 7,
///		Some(b'8') => num += 8,
///		Some(b'9') => break 'begin num,
///		Some(a) => panic!("Unk byte '{:?}'", a),
///		_ => break 'begin num,
///	});
///	
///	assert_eq!(a, Some(&b'9'));
///	assert_eq!(result, 36);
///}
///```
#[macro_export]
macro_rules! for_match {
	//new let iter, _
	[ $(@$loop_prefix:tt)?	 ($($args:tt)*) -> || { $($data:tt)* } ] => {{
		$crate::for_match_begin! {
			(
				[$($loop_prefix)?]
			):
			
			[$($args)*]
			[]
			
			{
				$($data)*
			}
		}
	}};

	[ $(@$loop_prefix:tt)?	 ($($args:tt)*) -> |$($names:tt),*| { $($data:tt)* } ] => {{
		$crate::for_match_begin! {
			(
				[$($loop_prefix)?]
			):
			
			[$($args)*]
			[$($names),*] 
			
			{
				$($data)*
			}
		}
	}};
}


#[doc(hidden)]
#[macro_export]
macro_rules! for_match_begin {
	//args
	[($($all_tt:tt)*): [$iter:ident] [$($names:tt)*] {$($data:tt)*}] => {
		$crate::for_match_begin! {
			($($all_tt)*): 
			
			[$iter, let mut _aa]
			[$($names)*] 
			
			{
				$($data)*
			}
		}
	};
	[($($all_tt:tt)*): [$iter:expr] [$($names:tt)*]	{$($data:tt)*}] => {
		$crate::for_match_begin! {
			($($all_tt)*): 
			
			[$iter, let mut _aa]
			[$($names)*] 
			
			{
				$($data)*
			}
		}
	};
	[($($all_tt:tt)*): [$iter:ident, let mut $a:ident $($args:tt)*] [$($names:tt)*] {$($data:tt)*}] => {
		let mut $a;
		$crate::for_match_begin! {
			($($all_tt)*): 
			
			[$iter, $a $($args)*]
			[$($names)*] 
			
			{
				$($data)*
			}
		}
	};
	[($($all_tt:tt)*): [$iter:expr, let mut $a:ident $($args:tt)*] [$($names:tt)*] {$($data:tt)*}] => {
		let mut $a;
		$crate::for_match_begin! {
			($($all_tt)*): 
			
			[$iter, $a $($args)*]
			[$($names)*] 
			
			{
				$($data)*
			}
		}
	};
	[($($all_tt:tt)*): [$iter:ident, _ $($args:tt)*] [$($names:tt)*]	{$($data:tt)*}] => {
		$crate::for_match_begin! {
			($($all_tt)*): 
			
			[$iter, let mut _aa $($args)*]
			[$($names)*] 
			
			{
				$($data)*
			}
		}
	};
	[($($all_tt:tt)*): [$iter:expr, _ $($args:tt)*] [$($names:tt)*]	{$($data:tt)*}] => {
		$crate::for_match_begin! {
			($($all_tt)*): 
			
			[$iter, let mut _aa $($args)*]
			[$($names)*] 
			
			{
				$($data)*
			}
		}
	};
	/*[($($all_tt:tt)*): [$iter:ident, _ $($args:tt)*]~[$($names:tt)*]	] => {
		let mut aas;
		$crate::for_match_begin! {
			($($all_tt)*): 
			
			[$iter, aas $($args)*]~
			[$($names)*]
		}
	};*/
	
	
	//names
	[($($all_tt:tt)*): [$($args:tt)*] [] {$($data:tt)*}] => {
		$crate::for_match_begin! {
			($($all_tt)*): 
			
			[$($args)*]
			[__iiter] 
			
			{
				$($data)*
			}
		}
	};
	[($($all_tt:tt)*): [$($args:tt)*] [_ $($names:tt)*] {$($data:tt)*}] => {
		$crate::for_match_begin! {
			($($all_tt)*): 
			
			[$($args)*]
			[__iiter $($names)*] 
			
			{
				$($data)*
			}
		}
	};
	
	
	[//iter ident
		( [$($loop_prefix:tt)?] ): 
			[$iter:ident, $a:expr	$(, $nn:expr)* $(,)?] //args
			[$iter_name: ident 	$(, $nn_ident:tt)* $(,)?] //names
			{
				$($data:tt)*
			}
	] => {
		let mut $iter_name = $iter.iter();
		
		$crate::cycle_variable_init! {
			@new 
			
			{ $([$nn_ident]),* }
			{ $([$nn]),* }
		}
		
		$($loop_prefix:)? loop {
			$a = core::iter::Iterator::next(&mut $iter_name);
			$crate::cycle_matchbegin!(@for ($a): $($data)*);
		}
	};
	[//iter ident
		( [$($loop_prefix:tt)?] ): 
			[$iter:expr, $a:expr	$(, $nn:expr)* $(,)?] //args
			[$iter_name: ident 	$(, $nn_ident:tt)* $(,)?] //names
			
			{
				$($data:tt)*
			}
	] => {
		let mut $iter_name = $iter;
		
		$crate::cycle_variable_init! {
			@new 
			
			{ $([$nn_ident]),* }
			{ $([$nn]),* }
		}
		
		$($loop_prefix:)? loop {
			$a = core::iter::Iterator::next(&mut $iter_name);
			$crate::cycle_matchbegin!(@for ($a): $($data)*);
		}
	};
}

