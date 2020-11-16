#[derive(Debug, std::cmp::PartialEq, std::cmp::Eq)]
enum IntegerSign {
    Positive,
    Negative
}

#[derive(Debug, std::cmp::Eq, std::cmp::PartialEq)]
pub struct PrimitiveBigInt<InnerType> {
    sign: IntegerSign,
    contents: Vec<InnerType>
}

type WordType = u32;
type BigInt = PrimitiveBigInt<WordType>;

/***************
 * Conversions *
 ***************/

impl std::convert::From<i32> for BigInt {
    fn from(i: i32) -> Self {
	let s: IntegerSign = match i < 0 {
	    true => IntegerSign::Negative,
	    false => IntegerSign::Positive
	};
	let ret = BigInt {
	    sign: s,
	    contents: vec![i.wrapping_abs() as WordType]
	};
	return ret;
    }
}

/***************
 * Comparisons *
 ***************/

impl std::cmp::PartialOrd for IntegerSign {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
	return Some(match self {
	    IntegerSign::Positive => {
		match other {
		    IntegerSign::Positive => std::cmp::Ordering::Equal,
		    IntegerSign::Negative => std::cmp::Ordering::Greater
		}
	    },
	    IntegerSign::Negative => {
		match other {
		    IntegerSign::Positive => std::cmp::Ordering::Less,
		    IntegerSign::Negative => std::cmp::Ordering::Equal
		}
	    }
	});
    }
}

impl std::cmp::Ord for IntegerSign {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
	return self.partial_cmp(&other).unwrap();
    }
}

impl std::cmp::PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
	// Check signs
	let sign_cmp = self.sign.cmp(&other.sign);
	if sign_cmp != std::cmp::Ordering::Equal {
	    return Some(sign_cmp);
	}
	// Break early if lengths are not the same
	let len_cmp = self.contents.len().cmp(&other.contents.len());
	if len_cmp != std::cmp::Ordering::Equal {
	    return Some(len_cmp);
	}
	// Check pairwise, in order of most significant to least significant
	for (left_word, right_word) in self.contents.iter().zip(other.contents.iter()).rev() {
	    let val_cmp = left_word.cmp(&right_word);
	    if val_cmp != std::cmp::Ordering::Equal {
		if self.sign == IntegerSign::Negative {
		    // Invert magnitude comparisons for negative numbers
		    return Some(match val_cmp {
			std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
			std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
			_ => std::cmp::Ordering::Equal
		    });
		}
		return Some(val_cmp);
	    }
	}
	return Some(std::cmp::Ordering::Equal);
    }
}

impl std::cmp::PartialEq<i32> for BigInt {
    fn eq(&self, other: &i32) -> bool {
	return self.eq(&BigInt::from(*other));
    }
}

impl std::cmp::Ord for BigInt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
	return self.partial_cmp(&other).unwrap();
    }
}

/*************
 * Operators *
 *************/

// Add
/*
impl core::ops::Add<BigInt> for BigInt {
    type Output = BigInt;
    fn add(self, rhs: BigInt) -> BigInt {
	
    }
}

impl core::ops::Add<i32> for BigInt {
    type Output = BigInt;
    fn add(self, _rhs: i32) -> BigInt {
	 
    }
}
*/

// AddAssign
// BitAnd
// BitAndAssign
// BitOr
// BitOrAssign
// BitXor
// BitXorAssign
// Div
// DivAssign
// Mul
// MulAssign
// Neg
// Not
// Rem
// RemAssign
// Shl
// ShlAssign
// Shr
// ShrAssign
// Sub
// SubAssign

impl BigInt {
    fn new() -> BigInt {
	return BigInt {
	    sign: IntegerSign::Positive,
	    contents: Vec::<WordType>::new()
	};
    }
    fn assign(&mut self, val: i32) {
	*self = BigInt::from(val);
    }
}

#[cfg(test)]
mod ord_cmp_tests {
    use super::*;
    
    #[test]
    fn convert_compare_i32() {
	let val: i32 = 17;
	let mut bi: BigInt = BigInt::new();
	bi.assign(val);
	assert!(bi == val);
	let val: i32 = -258;
	bi.assign(val);
	assert!(bi == val);
    }

    #[test]
    fn compare() {
	let small_negative: i32 = -15;
	let large_negative: i32 = -451264;
	let small_positive: i32 = 44;
	let large_positive: i32 = 90144;
	let mut big_first: BigInt = BigInt::new();
	let mut big_second: BigInt = BigInt::new();
	// T1
	big_first.assign(small_negative);
	big_second.assign(large_negative);
	assert!(big_first > big_second);
	// T2
	big_first.assign(large_negative);
	assert!(big_first == big_second);
	// T3
	// T4
    }
}

#[cfg(test)]
mod math_tests {
    use super::*;
}
