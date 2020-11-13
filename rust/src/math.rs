pub fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first;
    let mut min = second;
    if min > max {
	min = first;
	max = second;
    }
    if min == 0 {
	return max;
    }
    loop {
	let res = max % min;
	if res == 0 {
	    return min;
	}
	max = min;
	min = res;
    }
}

pub fn abs(mut q: i64) -> u64{
    if q < 0 {
	q *= -1;
    }
    return q as u64;
}
