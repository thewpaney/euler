pub fn check_pythagorean_triplet(m: u64, n: u64) -> Option<(u64, u64, u64)> {
    if m < n {
	return None;
    }
    let p: u64 = (m * m) - (n * n);
    let q: u64 = 2 * m * n;
    let r: u64 = (m * m) + (n * n);
    if (p * p) + (q * q) == (r * r) {
	return Some((p, q, r));
    } else {
	return None;
    }
}
