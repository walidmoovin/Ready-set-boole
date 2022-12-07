// We are doing evaluator for a propositional formula in reverse polish notation
// There are 3 way to express arithmetic expressions:
// 3. postfix notation: 1 2 + (operators are after operands) = reverse polish notation
// 1. infix notation: 1 + 2 (operators are in between operands)
// 2. prefix notation: + 1 2 (operators are before operands) = polish notation
fn eval_formula(formula: &str) -> bool {
}

fn main() {
	let formula = "10&";
	let result = eval_formula(formula);
	println!("{} = {}", formula, result);
}