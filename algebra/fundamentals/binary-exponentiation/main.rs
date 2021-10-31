fn recursive_binary_pow(base: u64, exponent: u64) -> u64 {
	if exponent == 0 {
		return 1;
	}

	let result = recursive_binary_pow(base, exponent >> 1);
	if exponent & 1 == 1{
		return result * result * base;
	} else {
		return result * result
	}
}

fn binary_pow(mut base: u64, mut exponent: u64) -> u64 {
	let mut result = 1;

	while exponent > 0 {
		if exponent & 1 == 1 {
			result *= base;
		}
		base *= base;
		exponent >>= 1;
	}

	result
}

fn modular_binary_pow(mut base: u64, mut exponent: u64, modulo: u64) -> u64 {
	let mut result = 1;
	base %= modulo;

	while exponent > 0 { 
		if exponent & 1 == 1 {
			result *= base % modulo;
		}
		base *= base % modulo;
		exponent >>= 1;
	}

	result % modulo
}

fn main() {
	assert_eq!(binary_pow(3, 13), 1594323);
	assert_eq!(recursive_binary_pow(3, 13), 1594323);
	assert_eq!(modular_binary_pow(3, 13, 100), 23);
}