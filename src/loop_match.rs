

/// [DOC IS ATTACHED!] The `For` loop, combined with matching.
#[macro_export]
macro_rules! loop_match {
	//let mut
	
	[ $(@$loop_prefix:tt)?		($($e_l:tt)*) -> || { $($tt:tt)* } ] => {{
		$crate::loop_match_begin! {
			(
				[$(@$loop_prefix)?]~
				[$($e_l)*]~
				[$($tt)*]
			):
		}
	}};
	
	[ $(@$loop_prefix:tt)?		($($e_l:tt)*) -> |$($nn:tt),*| { $($tt:tt)* } ] => {{
		$crate::loop_match_begin! {
			(
				[$(@$loop_prefix)?]~
				[$($e_l)*]~
				[$($tt)*]
			):
			
			$($nn),*
		}
	}};
}


#[doc(hidden)]
#[macro_export]
macro_rules! loop_match_begin {
	[	
		([$(@$loop_prefix:tt)?]~[ $a:expr $(, $nn_e:expr)* $(,)? ]~[$($tt:tt)*]): 
			$($nn:tt),*
	] => {{
		$crate::cycle_variable_init! {
			$([$nn_e]: $nn)*
		};
		
		$($loop_prefix:)? loop {
			$crate::cycle_matchbegin!(@loop ($a): $($tt)*);
		}
	}};
	[	
		([$(@$loop_prefix:tt)?]~[ $a:ident $(, $nn_e:expr)* $(,)? ]~[$($tt:tt)*]): 
			$($nn:tt),*
	] => {{
		$crate::cycle_variable_init! {
			$([$nn_e]: $nn)*
		};
		
		$($loop_prefix:)? loop {
			$crate::cycle_matchbegin!(@loop ($a): $($tt)*);
		}
	}};
}


