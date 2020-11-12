/* 
 * Problem : 10,001st prime
 * By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
 * What is the 10,001st prime number?
 */ 

use common::primes;

fn main() {
    let mut prime_count = 0;
    for n in 2..u64::MAX {
	if primes::is_prime_ascending(n) {
	    if prime_count == 10000 {
		println!("{}", n);
		break;
	    } else {
		prime_count += 1;
	    }
	}
    }
}
