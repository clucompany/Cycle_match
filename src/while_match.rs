

/// [DOC IS ATTACHED!] The `while` loop, combined with matching.
#[macro_export]
macro_rules! while_match {
	[ $(@$loop_prefix:tt)?		($($args:tt)*) -> || { $($data:tt)* } ] => {{
		$crate::while_match_begin! {
			([$($loop_prefix)?]):
			
			[$($args)*]
			[] 
			
			{
				$($data)*
			}
		}
	}};
	
	[ $(@$loop_prefix:tt)?		($($args:tt)*) -> |$($name_tt:tt),*| { $($data:tt)* } ] => {{
		$crate::while_match_begin! {
			([$($loop_prefix)?]):
			
			[$($args)*]
			[$($name_tt),*] 
			
			{
				$($data)*
			}
		}
	}};
	//
}


#[doc(hidden)]
#[macro_export]
macro_rules! while_match_begin {
	//$iter args -> full args
	[($($all_tt:tt)*): [$iter:ident] [$($names:tt)*] {$($data:tt)*} ] => {
		$crate::while_match_begin! {
			($($all_tt)*): [$iter, let mut _aa] [$($names)*] {
				$($data)*
			}
		}
	};
	[($($all_tt:tt)*): [$iter:expr] [$($names:tt)*] {$($data:tt)*} ] => {
		$crate::while_match_begin! {
			($($all_tt)*): [$iter, let mut _aa] [$($names)*] {
				$($data)*
			}
		}
	};
	
	//decode args
	[($($all_tt:tt)*): [$iter:ident, let mut $a:ident $($args:tt)*] [$($names:tt)*] {$($data:tt)*} ] => {
		let mut $a;
		$crate::while_match_begin! {
			($($all_tt)*): [$iter, $a $($args)*] [$($names)*] {
				$($data)*
			}
		}
	};
	[($($all_tt:tt)*): [$iter:expr, let mut $a:ident $($args:tt)*] [$($names:tt)*] {$($data:tt)*} ] => {
		let mut $a;
		$crate::while_match_begin! {
			($($all_tt)*): [$iter, $a $($args)*] [$($names)*] {
				$($data)*
			}
		}
	};
	[($($all_tt:tt)*): [$iter:ident, _ $($args:tt)*] [$($names:tt)*]  {$($data:tt)*} ] => {
		$crate::while_match_begin! {
			($($all_tt)*): [$iter, let mut _aa $($args)*] [$($names)*] {
				$($data)*
			}
		}
	};
	[($($all_tt:tt)*): [$iter:expr, _ $($args:tt)*] [$($names:tt)*] {$($data:tt)*} ] => {
		$crate::while_match_begin! {
			($($all_tt)*): [$iter, let mut _aa $($args)*] [$($names)*] {
				$($data)*
			}
		}
	};
	
	[
		([$($loop_prefix:tt)?]): 
			[$iter:ident, $a:ident $(,$nn:expr)*	$(,)?]
			[$($nn_ident:tt),*				$(,)?] 
			
			{
				$($data:tt)*
			}
	] => {
		$crate::cycle_variable_init! {
			@new 
			
			{ $([$nn_ident]),* }
			{ $([$nn]),* }
		}
		
		$($loop_prefix:)? loop {
			$a = core::iter::Iterator::next(&mut $iter);
			$crate::cycle_matchbegin!(@while ($a): $($data)*);
		}
	};
	
	[
		([$($loop_prefix:tt)?]): 
			[$iter:expr, $a:ident $(, $nn:expr)*	$(,)?]
			[$( $nn_ident:tt ),*				$(,)?] 
			{
				$($data:tt)*
			}
	] => {
		$crate::cycle_variable_init! {
			@new 
			
			{ $([$nn_ident]),* }
			{ $([$nn]),* }
		}
		
		$($loop_prefix:)? loop {
			$a = $iter;
			$crate::cycle_matchbegin!(@while ($a): $($data)*);
		}
	};
}

