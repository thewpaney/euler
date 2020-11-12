/* 
 * Problem 5: Smallest multiple
 * 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
 * What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20? 
 */ 


fn main() {
    let mut n = 2520;
    let n_max: u64 = 2_432_902_008_176_640_000;  // 20! just barely fits in a u64
    while n < n_max {
	for i in 1..=21 {
	    if i == 21 {
		// We made it through 20 - exit with success
		println!("{}", n);
		return;
	    }
	    if n % i != 0 {
		n += 1;
		break;
	    }
	}
    }
}
