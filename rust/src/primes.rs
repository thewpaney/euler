use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use serde_derive::{Serialize, Deserialize};

use crate::search;

static PRIME_LIST_FILENAME: &str = "primes.txt";
static PRIME_LIST_MAX: u64 = 1_000_000;  // One billion, mwahaha

pub fn is_prime_ascending(n: u64) -> bool {
    for i in 2..(n/2) {
	if n % i == 0 {
	    return false;
	}
    }
    return true;
}

pub fn is_prime_descending(n: u64) -> bool {
    for i in (2..(n/2)).rev() {
	if n % i == 0 {
	    return false;
	}
    }
    return true;
}

pub fn is_prime_list(n: u64) -> bool {
    println!("Checking {}...", n);
    let prime_list = get_prime_list();
    return match search::bsearch_vec(prime_list, n) {
	Some(_) => true,
	None => false
    };
}

pub fn nth_prime(n: usize) -> u64 {
    let prime_list = get_prime_list();
    if prime_list.len() < n {
	panic!("Need a longer prime list!");
    }
    return prime_list[n];
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct PrimeList(Vec<u64>);

fn get_prime_list() -> Vec<u64> {
    return get_prime_list_manual(PRIME_LIST_MAX);
}

pub fn get_prime_list_manual(max: u64) -> Vec<u64> {
    // Check if file exists
    if !Path::new(PRIME_LIST_FILENAME).exists() {
	generate_prime_list_eratosthenes(max);
    }
    let mut file_in = File::open(PRIME_LIST_FILENAME).unwrap();
    let mut encoded_prime_list: Vec<u8> = Vec::new();
    file_in.read_to_end(&mut encoded_prime_list).unwrap();
    let prime_list: PrimeList = bincode::deserialize(&encoded_prime_list[..]).unwrap();
    return prime_list.0;
}

fn generate_prime_list_eratosthenes(input_upper_bound: u64) {
    let sieve_window_length: usize = input_upper_bound as usize;
    if (sieve_window_length as u64) < input_upper_bound {
	eprintln!("Truncating upper bound from {} to {}", input_upper_bound, sieve_window_length);
    }
    let mut sieve_window: Vec<bool> = vec![true; sieve_window_length];  // RAM warning
    let root_bound: usize = ((sieve_window_length as f64).sqrt() + 1.0) as usize;

    for i in 2..(root_bound - 1) {
	if sieve_window[i] {
	    let mut j: usize = i * i;
	    while j < sieve_window_length {
		sieve_window[j] = false;
		j += i;
	    }
	}
    }

    let mut prime_list: Vec<u64> = Vec::new();
    let mut prime_index: usize = 2;
    while prime_index < sieve_window_length {
	if sieve_window[prime_index] {
	    prime_list.push(prime_index as u64);
	}
	prime_index += 1;
    }

    let encoded_prime_list: Vec<u8> = bincode::serialize(&prime_list).unwrap();
    let mut file_out = File::create(PRIME_LIST_FILENAME).unwrap();
    file_out.write_all(&encoded_prime_list).unwrap();
    println!("Prime list written to {}.", PRIME_LIST_FILENAME);
    return;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_list_generation() {
	assert_eq!(is_prime_list(8), false);
	assert_eq!(is_prime_list(991), true);
	assert_eq!(is_prime_list(7), true);
    }
}
