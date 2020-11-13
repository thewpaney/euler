pub fn bsearch_vec<T>(list: Vec<T>, target: T) -> Option<usize>
where T: std::cmp::PartialOrd + std::fmt::Display
{
    let mut min: usize = 0;
    let mut max: usize = list.len();
    let mut partition: usize = max / 2;
    loop {
	if list[partition] == target {
	    // Found it!
	    return Some(partition);
	} else if (max - min) == 1 {
	    // One left, and it's not the element we seek
	    return None;
	} else if list[partition] < target {
	    // Move right
	    min = partition;
	} else if list[partition] > target {
	    // Move left
	    max = partition;
	}
	partition = (min + max) / 2;
    }
}
