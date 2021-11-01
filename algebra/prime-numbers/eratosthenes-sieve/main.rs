fn sieve(size: usize) -> Vec<usize> {
	let mut is_prime = vec![true; size];
	let mut primes: Vec<usize> = Vec::new();

	for number in 2..size {
		if is_prime[number] {
			primes.push(number);

			for multiplier in 2..(size / number) {
				is_prime[number * multiplier] = false;
			}
		}
	}

	primes
}

fn main() {
	assert_eq!(sieve(10), vec![2, 3, 5, 7, 9])
}
