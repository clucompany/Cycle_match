

/// [DOC IS ATTACHED!] The `For` loop, combined with matching.
#[macro_export]
macro_rules! loop_match {
	//let mut
	
	[ $(@$loop_prefix:tt)?		($($args:tt)*) -> || { $($data:tt)* } ] => {{
		$crate::loop_match_begin! {
			(
				[$($loop_prefix)?]
				[$($args)*]
				[]
			) {
				$($data)*
			}
		}
	}};
	
	[ $(@$loop_prefix:tt)?		($($args:tt)*) -> |$($nn:tt),*| { $($data:tt)* } ] => {{
		$crate::loop_match_begin! {
			(
				[$($loop_prefix)?]
				[$($args)*]
				[$($nn),*]
			) {
				$($data)*
			}
		}
	}};
}


#[doc(hidden)]
#[macro_export]
macro_rules! loop_match_begin {
	[	
		([$($loop_prefix:tt)?][ $a:expr $(, $nn:expr)* $(,)? ][$($nn_ident:tt),*] $(,)?) {
			$($data:tt)*
		}
	] => {{
		$crate::cycle_variable_init! {
			@new 
			
			{ $([$nn_ident]),* }
			{ $([$nn]),* }
		}
		
		$($loop_prefix:)? loop {
			$crate::cycle_matchbegin!(@loop ($a): $($data)*);
		}
	}};
	[	
		([$($loop_prefix:tt)?][ $a:ident $(, $nn:expr)* $(,)? ][$($nn_ident:tt),*] $(,)?) {
			$($data:tt)*
		}
	] => {{
		$crate::cycle_variable_init! {
			@new 
			
			{ $([$nn_ident]),* }
			{ $([$nn]),* }
		}
		
		$($loop_prefix:)? loop {
			$crate::cycle_matchbegin!(@loop ($a): $($data)*);
		}
	}};
}


