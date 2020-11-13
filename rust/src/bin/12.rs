/* 
 * Problem 12: Highly divisible triangular number
 */ 

use common::factoring;

fn main() {
    // Brute force!
    // Generate triangular numbers until one has more than 500 divisors
    let mut t: u64 = 0;
    let mut count: u64 = 0;
    loop {
	count += 1;
	t += count;

	let factors = factoring::get_factors_simple(t);
	
	if factors.len() > 500 {
	    println!("T{} = {} has {} factors: {:?}", count, t, factors.len(), factors);
	    break;
	}
    }
}
