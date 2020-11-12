pub fn is_palindrome(n: u64) -> bool {
    let mut original: u64 = n;
    let mut new: u64 = 0;
    while original > 0 {
	new *= 10;
	new += original % 10;
	original /= 10;
    }
    return n == new;
}
