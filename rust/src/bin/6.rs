/* 
 * Problem 6: Sum of square difference
 * The sum of the squares of the first ten natural numbers is 385.
 * The square of the sum of the first ten natural numbers is 3025.
 * Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 - 385 = 2640.
 * Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
 */ 


fn main() {
    let mut sum_of_squares: u64 = 0;
    let mut sum: u64 = 0;
    for n in 1..=100 {
	sum += n;
	sum_of_squares += n * n;
    }
    let square_of_sum: u64 = sum * sum;
    println!("{}", square_of_sum - sum_of_squares);
}
