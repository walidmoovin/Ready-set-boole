// We are making a function that takes a str containing a formula in reverse polish notation
// It returns its truth table on the standard output

fn eval_formula(formula: &str) -> bool {
	let mut stack = Vec::new();
	for c in formula.chars() {
		match c {
			'0' => stack.push(false),
			'1' => stack.push(true),
			'&' => {
				let a = stack.pop().unwrap();
				let b = stack.pop().unwrap();
				stack.push(a & b);
			},
			'|' => {
				let a = stack.pop().unwrap();
				let b = stack.pop().unwrap();
				stack.push(a | b);
			},
			'!' => {
				let a = stack.pop().unwrap();
				stack.push(!a);
			},
			'^' => {
				let a = stack.pop().unwrap();
				let b = stack.pop().unwrap();
				stack.push(a ^ b);
			},
			'>' => {
				let a = stack.pop().unwrap();
				let b = stack.pop().unwrap();
				stack.push(!a | b);
			},
			'=' => {
				let a = stack.pop().unwrap();
				let b = stack.pop().unwrap();
				stack.push(a == b);
			},
			_ => (),
		}
	}
	return stack.pop().unwrap();
}

// | A | B | C | = |$
// |---|---|---|---|$
// | 0 | 0 | 0 | 0 |$
// | 0 | 0 | 1 | 1 |$
// | 0 | 1 | 0 | 0 |$
// | 0 | 1 | 1 | 1 |$
// | 1 | 0 | 0 | 0 |$
// | 1 | 0 | 1 | 1 |$
// | 1 | 1 | 0 | 1 |$
// | 1 | 1 | 1 | 1 |$
// We have to print the truth table of a formula like the one above

fn print_truth_table(formula: &str) {
}

fn main() {
	let formula = "AB&C|";
	print_truth_table(formula);
}