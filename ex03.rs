// We are doing evaluator for a propositional formula in reverse polish notation
// There are 3 way to express arithmetic expressions:
// 3. postfix notation: 1 2 + (operators are after operands) = reverse polish notation
// 1. infix notation: 1 + 2 (operators are in between operands)
// 2. prefix notation: + 1 2 (operators are before operands) = polish notation
// https://www.youtube.com/watch?v=qN8LPIcY6K
// to evaluate a formula, we use a stack
// we push operands on the stack
// when we encounter an operator, we pop the last 2 operands and apply the operator
// we push the result back on the stack
// when we have no more operators, we pop the last operand and it's the result

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

fn main() {
	let formula = "101|&";
	let result = eval_formula(formula);
	println!("{} = {}", formula, result);
}