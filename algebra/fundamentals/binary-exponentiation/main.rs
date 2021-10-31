#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}

fn recursive_binary_pow(base: u64, exponent: u64) -> u64 {
	if exponent == 0 {
		return 1;
	}

	let result = recursive_binary_pow(base, exponent >> 1);
	if exponent & 1 {
		return result * result * base;
	} else {
		return result * result
	}
}

fn main() {
	println!("Hello")
}