// We are doing a bitwise adder
// & operator helps with the carries in the addition
// ^ operator = sum of bits
// << operator = turn carry into a bit

// We have addition of 2 bits (0001, 0011)
// First we apply & operator on both to get the carry (0001 & 0011 = 0001)
// Then we apply ^ operator on both to get the sum (0001 ^ 0011 = 0010)
// Then we apply << operator on the carry to get the carry in the next bit (0001 << 1 = 0010)
// We repeat the process until we have no more carries

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
fn main() {
	let a = 1;
	let b = 2;
	let c = adder(a, b);
	println!("{} + {} = {}", a, b, c);
}