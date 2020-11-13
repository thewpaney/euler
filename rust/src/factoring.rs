use rand::{thread_rng, Rng};

use crate::{math, primes};

pub fn get_prime_factors(n: u64) -> Vec<u64> {
    let mut factors = factor_to_vec(n);
    let mut i: usize = 0;
    while i < factors.len() {
	if !primes::is_prime_list(factors[i]) {
	    let mut new_factors = factor_to_vec(factors[i]);
	    while new_factors[0] == factors[i] {
		new_factors = factor_to_vec(factors[i]);
	    }
	    factors.extend(new_factors);
	}
	i += 1;
    }
    factors.push(1);
    factors.push(n);
    factors.sort_unstable();
    factors.dedup();
    return factors;
}

pub fn get_factors_simple(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();
    let root: u64 = ((n as f64).sqrt() + 1.0) as u64;
    for i in 1..root {
	if n % i == 0 {
	    factors.push(i);
	    factors.push(n / i);
	}
    }
    return factors;
}

fn factor_to_vec(mut n: u64) -> Vec<u64> {
    let orig_n = n;
    let mut out: Vec<u64> = Vec::new();
    while n != 1 {
	let factor: u64 = brent_modified_pollard_rho_factor(n);
	n /= factor;
	out.push(factor);
    }
    println!("{} -> {:?}", orig_n, out);
    return out;
}

fn brent_modified_pollard_rho_factor(n: u64) -> u64{
    // ACK: https://maths-people.anu.edu.au/~brent/pd/rpb051i.pdf
    // ACK: https://comeoncodeon.wordpress.com/2010/09/18/pollard-rho-brent-integer-factorization/
    if n % 2 == 0 {
	return 2;
    }
    let mut rng = thread_rng();
    let mut y: u64 = rng.gen_range(1, n);
    let c: u64 = rng.gen_range(1, n);
    let m: u64 = rng.gen_range(1, n);
    let mut g: u64 = 1;
    let mut r: u64 = 1;
    let mut q: u64 = 1;
    let mut ys: u64 = y;
    let mut x: u64 = y;
    while g == 1 {
	x = y;
	for _ in 0..r {
	    y = (((y * y) % n) + c) % n;
	}
	let mut k: u64 = 0;
	while (k < r) && (g == 1) {
	    ys = y;
	    let min: u64 = *vec![m, r - k].iter().min().unwrap();
	    for _ in 0..min {
		y = (((y * y) % n) + c) % n;
		q = (q * math::abs(x as i64 - y as i64)) % n;
	    }
	    g = math::gcd(q, n);
	    k += m;
	}
	r *= 2;
    }
    if g == n {
	loop {
	    ys = (((ys * ys) % n) + c) % n;
	    g = math::gcd(math::abs(x as i64 - ys as i64), n);
	    if g > 1 {
		break;
	    }
	}
    }
    return g;
}
