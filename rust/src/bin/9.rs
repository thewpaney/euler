/* 
 * Problem 9: A Pythagorean triplet is a set of three natural numbers, a < b < c, for which, a^2 + b^2 = c^2.
 * For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
 * There exists exactly one Pythagorean triplet for which a + b + c = 1000. Find the product abc.
 */ 

use common::geometry;

fn main() {
    // Arbitrary search start and stop
    let start: u64 = 1;
    let stop: u64 = 50;
    for m in start..stop {
	for n in start..=m {
	    if let Some((p, q, r)) = geometry::check_pythagorean_triplet(m, n) {
		if p + q + r == 1000 {
		    println!("{}", p * q * r);
		    return;
		}
	    }
	}
    }
}
