/* 
 * Problem 700: Eulercoin
 * 
 */ 

fn main() {
    let eulercoin_start: u64 = 1_504_170_715_041_707;
    let eulercoin_mod: u64 = 4_503_599_627_370_517;
    let mut eulercoin_running: u64 = eulercoin_start;
    let mut mid_eulercoin: u64 = eulercoin_start;
    let mut min_eulercoin: u64 = eulercoin_start;
    let mut max_eulercoin: u64 = eulercoin_start;
    loop {
	if min_eulercoin == 1 {
	    break;
	}
	mid_eulercoin = (min_eulercoin + max_eulercoin) % eulercoin_mod;
	if mid_eulercoin > max_eulercoin {
	    max_eulercoin = mid_eulercoin;
	}
	if mid_eulercoin < min_eulercoin {
	    min_eulercoin = mid_eulercoin;
	    eulercoin_running += min_eulercoin;
	}
	println!("TOTAL: {}", eulercoin_running);
    }
}
