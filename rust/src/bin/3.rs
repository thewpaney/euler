/* 
 * Problem 3: Largest prime factor
 * The prime factors of 13195 are 5, 7, 13 and 29.
 * What is the largest prime factor of the number 600851475143?
 */ 

use common::primes;

fn main() {
    let mut factor_target: u64 = 600_851_475_143;
    let mut largest_prime_factor = 0;
    while factor_target > 1 {
	let mut factor = 0;
	for potential_factor in 2..factor_target {
	    // Count up - find a divisor
	    if (factor_target % potential_factor == 0) && primes::is_prime_ascending(factor) {
		factor = potential_factor;
		break;
	    }
	}
	if factor == 0 {
	    // factor_target is prime!
	    factor = factor_target;
	}
	println!("Factor: {}", factor);
	factor_target /= factor;
	println!("New target: {}", factor_target);
	if factor > largest_prime_factor {
	    largest_prime_factor = factor;
	}
    }
    println!("Largest prime factor: {}", largest_prime_factor)
}
