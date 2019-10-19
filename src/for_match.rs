
#[macro_export]
macro_rules! for_match {
	//new let iter, _
	[ $(@$loop_prefix:tt)?	 ($($new_iter:tt)*) -> |$($names:tt),*| { $($tt:tt)* } ] => {{
		$crate::for_match_begin! {
			(
				[$(@$loop_prefix)?]~
				[$($new_iter)*]~
				[$($tt)*]
			):
			
			$($names),*
		};
	}};
	[ $(@$loop_prefix:tt)?	 ($($new_iter:tt)*) -> |let mut $l:ident| { $($tt:tt)* } ] => {{
		$crate::for_match_begin! {
			(
				[$(@$loop_prefix)?]~
				[$($new_iter)*]~
				[$($tt)*]
			):
			
			let mut $l
		};
	}};
	[ $(@$loop_prefix:tt)?	 ($($new_iter:tt)*) -> |$name:ident, let mut $l:ident| { $($tt:tt)* } ] => {{
		$crate::for_match_begin! {
			(
				[$(@$loop_prefix)?]~
				[$($new_iter)*]~
				[$($tt)*]
			):
			
			$name, let mut $l
		};
	}};
	
}

#[doc(hidden)]
#[macro_export]
macro_rules! for_match_begin {
	[($($all_tt:tt)*): $iter_name:ident, _] => {{
		$crate::for_match_begin! {
			($($all_tt)*): $iter_name, let mut __aa
		}
	}};
	[($($all_tt:tt)*): $iter_name: ident, let mut $a:ident] => {{
		let mut $a;
		$crate::for_match_begin! {
			($($all_tt)*): $iter_name, $a
		}
	}};
	[($($all_tt:tt)*): let mut $a:ident] => {{
		let mut $a;
		$crate::for_match_begin! {
			($($all_tt)*): __iiter, $a
		}
	}};
	[($($all_tt:tt)*): _ ] => {{
		$crate::for_match_begin! {
			($($all_tt)*): __iiter, let mut __aa
		}
	}};
	[($($all_tt:tt)*): $a:ident] => {{
		$crate::for_match_begin! {
			($($all_tt)*): __iiter, $a
		}
	}};
	
	
	[([$(@$loop_prefix:tt)?]~[$($iter:ident)*]~[$($tt:tt)*]): $iter_name: ident, $a:ident] => {{
		let mut $iter_name = ($($iter)*).iter();
		$($loop_prefix:)? loop {
			$a = $iter_name.next();
			$crate::cycle_matchbegin!(@for ($a): $($tt)*);
		}
	}};
	
	[([$(@$loop_prefix:tt)?]~[$($iter:tt)*]~[$($tt:tt)*]): $iter_name: ident, $a:ident] => {{
		let mut $iter_name = ($($iter)*);
		$($loop_prefix:)? loop {
			$a = $iter_name.next();
			$crate::cycle_matchbegin!(@for ($a): $($tt)*);
		}
	}};
}

