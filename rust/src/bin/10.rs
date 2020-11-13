/* 
 * Problem 10: Summation of primes
 * The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
 * Find the sum of all the primes below two million.
 * 
 */ 

use common::primes;

fn main() {
    let prime_list: Vec<u64> = primes::get_prime_list_manual(2_000_000);  // Overriding the maximum
    let sum: u64 = prime_list.iter().sum();
    println!("{}", sum);
}
