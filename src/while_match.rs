

#[macro_export]
macro_rules! while_match {
	//let mut
	[ $(@$loop_prefix:tt)?		($($next:tt)+) -> |$name_tt:tt| { $($tt:tt)* } ] => {{
		$crate::while_match_begin! {
			(
				[$(@$loop_prefix)?]~
				[$($next)*]~
				[$($tt)*]
			):
			
			$name_tt
		};
	}};
	
	[ $(@$loop_prefix:tt)?		($($next:tt)+) -> |let mut $ident:ident| { $($tt:tt)* } ] => {{
		$crate::while_match_begin! {
			(
				[$(@$loop_prefix)?]~
				[$($next)*]~
				[$($tt)*]
			):
			
			let mut $ident
		};
	}};
	//
}


#[doc(hidden)]
#[macro_export]
macro_rules! while_match_begin {
	[($($all_tt:tt)*): _] => {{
		$crate::while_match_begin! {
			($($all_tt)*): let mut __aa
		}
	}};
	
	[($($all_tt:tt)*): let mut $a:ident] => {{
		let mut $a;
		$crate::while_match_begin! {
			($($all_tt)*): $a
		}
	}};
	
	[([$(@$loop_prefix:tt)?]~[$($next:ident)*]~[$($tt:tt)*]): $a:ident] => {{
		$($loop_prefix:)? loop {
			$a = ($($next)*).next();
			$crate::cycle_matchbegin!(@while ($a): $($tt)*);
		}
	}};
	
	[([$(@$loop_prefix:tt)?]~[$($next:tt)*]~[$($tt:tt)*]): $a:ident] => {{
		$($loop_prefix:)? loop {
			$a = ($($next)*);
			$crate::cycle_matchbegin!(@while ($a): $($tt)*);
		}
	}};
}

