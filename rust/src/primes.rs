use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use serde_derive::{Serialize, Deserialize};

static PRIME_LIST_FILENAME: &str = "primes.txt";
static PRIME_LIST_MAX: u64 = 1_000_000_000;  // One billion, mwahaha

pub fn is_prime_ascending(n: u64) -> bool {
    for i in (2..n/2) {
	if n % i == 0 {
	    return false;
	}
    }
    return true;
}

pub fn is_prime_descending(n: u64) -> bool {
    for i in (2..n/2).rev() {
	if n % i == 0 {
	    return false;
	}
    }
    return true;
}

pub fn is_prime_list(n: u64) -> bool {
    let prime_list = get_prime_list();
    println!("{} in {:?}", n, prime_list);
    return false;
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct PrimeList(Vec<u64>);

fn get_prime_list() -> Vec<u64> {
    // Check if file exists
    if !Path::new(PRIME_LIST_FILENAME).exists() {
	generate_prime_list();
    }
    let mut file_in = File::open(PRIME_LIST_FILENAME).unwrap();
    let mut encoded_prime_list: Vec<u8> = Vec::new();
    file_in.read_to_end(&mut encoded_prime_list).unwrap();
    let prime_list: PrimeList = bincode::deserialize(&encoded_prime_list[..]).unwrap();
    return prime_list.0;
}

fn generate_prime_list() {
    println!("Regenerating prime list...");
    let mut prime_list: Vec<u64> = Vec::new();
    for n in 2..PRIME_LIST_MAX {
	if is_prime_ascending(n) {
	    prime_list.push(n);
	}
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
	assert_eq!(is_prime_list(13), true);
	assert_eq!(is_prime_list(991), true);
    }
}
