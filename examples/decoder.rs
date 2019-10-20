
#[macro_use]
extern crate cycle_match;

fn main() {
	
	let data = "
/*
Привет, это мой текст....
*/
# ignore this txt,,,,,,, ......

Я его сейчас набираю, тестирую эту библиотеку. Потратил на нее два дня...
Слушаю уокера.

#Ulin 1819 cluCompany, ...
//...
19.10.2019 test.
";
	
	let mut q_comments = 0;
	
	let mut q_special_utf8 = 0;
	
	let mut q_points = 0;
	let mut q_commas = 0;
	let mut q_spaces = 0;
	
	
	let mut q_russian_utf8 = 0;
	let mut q_russian_big_utf8 = 0;
	let mut q_russian_small_utf8 = 0;
	
	
	let mut q_engl_utf8 = 0;
	let mut q_engl_big_utf8 = 0;
	let mut q_engl_small_utf8 = 0;
	
	let mut q_numbers = 0;
	
	for_match!(@'decoder (data.chars(), let mut a) -> |iter| {
		Some(' ') => q_spaces += 1,
		Some(',') => q_commas += 1,
		Some('.') => q_points += 1,
		Some('\n') | Some('\t') => q_special_utf8 += 1,
		Some('0' ..= '9') => q_numbers += 1,
		Some('А' ..= 'Я') | Some('Ё') => {
			q_russian_utf8 += 1;
			q_russian_big_utf8 += 1;
		},
		Some('а' ..= 'я') | Some('ё') => {
			q_russian_utf8 += 1;
			q_russian_small_utf8 += 1;
		},
		
		
		Some('A' ..= 'Z') => {
			q_engl_utf8 += 1;
			q_engl_big_utf8 += 1;
		},
		Some('a' ..= 'z') => {
			q_engl_utf8 += 1;
			q_engl_small_utf8 += 1;
		},
		
		Some('#') => {
			q_comments += 1;
			while_match!((iter, a) -> || {
				Some('\n') => continue 'decoder,
				Some(_a) => {},
				_ => break 'decoder,
			});
		},
		Some('/') => match iter.next() {
			Some('*') => {
				q_comments += 1;
				loop_match!(@'decode_a (a, a = iter.next()) -> |_| {
					Some('*') => match iter.next() {
						Some('/') => continue 'decoder,
						Some(_a) => {
							a = iter.next();
							continue 'decode_a;
						},
						_ => panic!("The symbol '/' was expected. "),
					},
					Some(_a) => {
						a = iter.next();
						continue 'decode_a;
					},
					_ => panic!("The symbol '*' was expected. "),
				});
			},
			Some('/') => {
				q_comments += 1;
				while_match!((iter, a) -> || {
					Some('\n') => continue 'decoder,
					Some(_a) => {},
					_ => break 'decoder,
				});
			},
			_ => panic!("The symbol '*' was expected. "),
		},
		
		Some(a) => panic!("Unk symbol '{}'", a),
		_ => break,
	});
	
	println!(
		"---------
q_comments:	{}
q_special:	{}
q_points:	{}
q_commas:	{}
q_spaces:	{}
---------
q_numbers:	{}
---------
q_russian:		{}
q_russian_big:	{}
q_russian_small:	{}
---------
q_engl:		{}
q_engl_big:		{}
q_engl_small:	{}
---------",
		
		q_comments,
		q_special_utf8,
		
		q_points,
		q_commas,
		q_spaces,
		
		q_numbers,
		
		q_russian_utf8,
		q_russian_big_utf8,
		q_russian_small_utf8,
		
		q_engl_utf8,
		q_engl_big_utf8,
		q_engl_small_utf8,
	);
}

