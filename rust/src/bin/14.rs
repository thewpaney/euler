/* 
 * Problem 14: Longest Collatz sequence
 * 
 */ 

fn collatz(mut start: u64) -> u64 {
    let mut len: u64 = 0;
    while start != 1 {
	if start % 2 == 0 {
	    start /= 2;
	} else {
	    start = start * 3 + 1;
	}
	len += 1;
    }
    return len;
}

fn main() {
    let mut longest: u64 = 0;
    let mut best: u64 = 0;
    for start in 1..1_000_000 {
	let length = collatz(start);
	if length > longest {
	    longest = length;
	    best = start;
	}
    }
    println!("Longest: {} (starting from {})", longest, best);
}
