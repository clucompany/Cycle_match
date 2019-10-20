

/// [DOC IS ATTACHED!] The `while` loop, combined with matching.
#[macro_export]
macro_rules! while_match {
	[ $(@$loop_prefix:tt)?		($($args:tt)*) -> || { $($data:tt)* } ] => {{
		$crate::while_match_begin! {
			(
				[$($loop_prefix)?]~
				[$($data)*]
			):
			
			[$($args)*]~[]
		}
	}};
	
	[ $(@$loop_prefix:tt)?		($($args:tt)*) -> |$($name_tt:tt),*| { $($data:tt)* } ] => {{
		$crate::while_match_begin! {
			(
				[$($loop_prefix)?]~
				[$($data)*]
			):
			
			[$($args)*]~[$($name_tt),*]
		}
	}};
	//
}


#[doc(hidden)]
#[macro_export]
macro_rules! while_match_begin {
	//args
	[($($all_tt:tt)*): [$iter:ident]~[$($names:tt)*] ] => {
		$crate::while_match_begin! {
			($($all_tt)*): [$iter, let mut _aa]~[$($names)*]
		}
	};
	[($($all_tt:tt)*): [$iter:expr]~[$($names:tt)*] ] => {
		$crate::while_match_begin! {
			($($all_tt)*): [$iter, let mut _aa]~[$($names)*]
		}
	};
	
	[($($all_tt:tt)*): [$iter:ident, let mut $a:ident $($args:tt)*]~[$($names:tt)*] ] => {
		let mut $a;
		$crate::while_match_begin! {
			($($all_tt)*): [$iter, $a $($args)*]~[$($names)*]
		}
	};
	[($($all_tt:tt)*): [$iter:expr, let mut $a:ident $($args:tt)*]~[$($names:tt)*] ] => {
		let mut $a;
		$crate::while_match_begin! {
			($($all_tt)*): [$iter, $a $($args)*]~[$($names)*]
		}
	};
	[($($all_tt:tt)*): [$iter:ident, _ $($args:tt)*]~[$($names:tt)*] ] => {
		$crate::while_match_begin! {
			($($all_tt)*): [$iter, let mut _aa $($args)*]~[$($names)*]
		}
	};
	[($($all_tt:tt)*): [$iter:expr, _ $($args:tt)*]~[$($names:tt)*] ] => {
		$crate::while_match_begin! {
			($($all_tt)*): [$iter, let mut _aa $($args)*]~[$($names)*]
		}
	};
	
	[
		([$($loop_prefix:tt)?]~[$($data:tt)*]): 
			[$iter:ident, $a:ident $(,$nn:expr)*	$(,)?]~
			[$($nn_ident:tt),*				$(,)?] 
	] => {
		$crate::cycle_variable_init! {
			$([$nn]: $nn_ident)*
		};
		
		$($loop_prefix:)? loop {
			$a = core::iter::Iterator::next(&mut $iter);
			$crate::cycle_matchbegin!(@while ($a): $($data)*);
		}
	};
	
	[
		([$($loop_prefix:tt)?]~[$($data:tt)*]): 
			[$iter:expr, $a:ident $(,$nn:expr)*		$(,)?]~
			[$($nn_ident:tt),*				$(,)?] 
	] => {
		$crate::cycle_variable_init! {
			$([$nn]: $nn_ident)*
		};
		
		$($loop_prefix:)? loop {
			$a = $iter;
			$crate::cycle_matchbegin!(@while ($a): $($data)*);
		}
	};
}

