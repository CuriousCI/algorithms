fn recursive_gcd(alpha: u64, beta: u64) -> u64 {
	match beta {
		0 => alpha,
		_ => recursive_gcd(beta, alpha % beta),
	}
}

fn gcd(mut alpha: u64, mut beta: u64) -> u64 {
	while beta > 0 {
		alpha %= beta;
		std::mem::swap(&mut alpha, &mut beta);
	}

	alpha
}

fn binary_gcd(mut alpha: u64, mut beta: u64) -> u64 {
	if alpha == 0 || beta == 0 {
		return alpha | beta;
	}

	let shift = (alpha | beta).trailing_zeros();
	alpha >>= alpha.trailing_zeros();

	loop {
		beta >>= beta.trailing_zeros();
		if alpha > beta {
			std::mem::swap(&mut alpha, &mut beta);
		}
		beta -= alpha;
		if beta <= 0 {
			break;
		}
	}

	alpha << shift
}

fn lcm(alpha: u64, beta: u64) -> u64 {
	alpha / recursive_gcd(alpha, beta) * beta
}

fn main() {
	assert_eq!(recursive_gcd(1, 1), 1);
	assert_eq!(recursive_gcd(10, 10), 10);
	assert_eq!(recursive_gcd(24, 0), 24);
	assert_eq!(recursive_gcd(56, 72), 8);
	assert_eq!(gcd(1, 1), 1);
	assert_eq!(gcd(10, 10), 10);
	assert_eq!(gcd(24, 0), 24);
	assert_eq!(gcd(56, 72), 8);
	assert_eq!(binary_gcd(1, 1), 1);
	assert_eq!(binary_gcd(10, 10), 10);
	assert_eq!(binary_gcd(24, 0), 24);
	assert_eq!(binary_gcd(56, 72), 8);
}
