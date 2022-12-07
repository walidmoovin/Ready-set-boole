// We are writing a function that takes an int n and returns its equivalent in gray code
// The problem with natural binary codes is that physical switches are not ideal: 
// In iterative process of a switch going 3->4, it will pass through a number of intermediate states.
// the transition might look like 011 — 001 — 101 — 100
// Switches are in position 001, observer cannot tell it's real or transitional state between two other positions.
// So systems that use binary codes are prone to errors.

// Binary to gray code: we compare each bit with the previous one and we write 1 if they are different, 0 otherwise

fn binary_to_gray(n: u32) -> u32 {
	let gray = n ^ (n >> 1);
	return gray;
}

fn main() {
	let n = 4;
	let gray = binary_to_gray(n);
	println!("{} in binary is {} in gray code", n, gray);
}