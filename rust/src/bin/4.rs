/* 
 * Problem 4: Largest palindrome product
 * A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
 * Find the largest palindrome made from the product of two 3-digit numbers.
 */ 

use common::digits;

fn main() {
    let mut biggest: u64 = 0;
    for n1 in (100..999).rev() {
	for n2 in (100..999).rev() {
//	    println!("Checking {}...", n1 * n2);
	    if digits::is_palindrome(n1 * n2) {
//		println!("{} * {} = {}", n1, n2, n1 * n2);
		if n1 * n2 > biggest {
		    biggest = n1 * n2;
		}
	    }
	}
    }
    println!("{}", biggest);
}
