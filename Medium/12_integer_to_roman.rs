// https://leetcode.com/problems/integer-to-roman/

pub fn romanize_digit(
	one: char,
	five: char,
	ten: char,
	mut digit: i32,
	out: &mut String
) {
	match digit {
		0 => (),
		4 => {
			out.push(one);
			out.push(five);
		}
		5 => out.push(five),
		9 => {
			out.push(one);
			out.push(ten);
		},
		_ => {
			if digit > 5 {
				out.push(five);
				digit -= 5;
			}
			for _i in 1..= digit {
				out.push(one);
			}
		}
	}
}

pub fn int_to_roman(mut num: i32) -> String {
	let mut out = String::from("");
	
	let thousands_place = num / 1000;
	for _i in 1..= thousands_place {
		out.push_str("M");
	}
	num %= 1000;
	
	Self::romanize_digit('C', 'D', 'M', num / 100 % 10, &mut out);
	Self::romanize_digit('X', 'L', 'C', num / 10 % 10, &mut out);
	Self::romanize_digit('I', 'V', 'X', num % 10, &mut out);
	out
}