// We are doing a bitwise multiplier
// it's basically a loop that adds the number to itself n times
// where n is the number of bits in the multiplier
// (x * y) = (x * 2) * (y / 2) + (x) if y is odd and (x * 2) * (y / 2) if y is even
// (x * y) = (x << 1) * (y >> 1) + (x) if y is odd and (x << 1) * (y >> 1) if y is even
// When y is odd, we add x to the result

fn adder(a: u32, b: u32) -> u32 {
	let mut sum = a ^ b; // sum of bits
	let mut carry = (a & b) << 1; // carry
	while carry != 0 { // if there is a carry, we repeat the process
		let temp = sum;
		sum = sum ^ carry;
		carry = (temp & carry) << 1;
	}
	return sum;
}

fn multiplier(a: u32, b: u32) -> u32 {
	let mut sum = 0;
	let mut carry = a;
	let mut multiplier = b;
	while multiplier != 0 {
		if multiplier & 1 == 1 { // verify if multiplier is odd (last bit is 1)
			sum = adder(sum, carry);
		}
		carry = carry << 1;
		multiplier = multiplier >> 1;
	}
	return sum;
}

fn main() {
	let a = 2;
	let b = 3;
	let c = multiplier(a, b);
	println!("{} * {} = {}", a, b, c);
}