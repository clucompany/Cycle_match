#[macro_use]
extern crate cycle_match;

#[test]
fn read_liner() {
	
	let data = b"# test: read_line
line 1
line 2
line 3
line 4
line 5
#end
line 6
#aa
line 7

";
	let mut comments_count = 0;
	let mut str_count = 0;
	
	let mut is_str = false;
	for_match!(@'read (data.into_iter()) -> |iter| {
		Some(13u8) => continue,
		Some(b'\n') => is_str = false,
		
		Some(b'#') => while_match!((iter, _, comments_count += 1) -> |_| {
			Some(b'\n') => continue 'read,
			Some(_a) => {},
			_ => break 'read,
		}),
		
		Some(_a) => if !is_str {
			is_str = true;
			str_count += 1;
		},
		_ => break,
	});
	
	assert_eq!(comments_count, 3);
	assert_eq!(str_count, 7);
}
