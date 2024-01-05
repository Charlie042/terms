use std::collections::HashMap;

fn nth_most_rarest(numbers: &[i32], n: usize) -> Option<i32> {
    // Count occurrences of each number in the list
    let mut frequency_map = HashMap::new();
    for &num in numbers {
        *frequency_map.entry(num).or_insert(0) += 1;
    }

    // Create a vector of unique frequencies
    let mut unique_frequencies: Vec<_> = frequency_map.values().collect();
    unique_frequencies.sort();

    // Find the nth rarest frequency
    if let Some(&nth_rarest_frequency) = unique_frequencies.get(unique_frequencies.len().saturating_sub(n)) {
        // Find the first number with the nth rarest frequency
        for (&num, frequency) in &frequency_map {
            if frequency == nth_rarest_frequency {
                return Some(num);
            }
        }
    }

    None
}

fn main() {
    let numbers = vec![5,4,5,4,5,4,4,5,3,3,3,3,2,2,1,5];
    let n = 3;

    if let Some(result) = nth_most_rarest(&numbers, n) {
        println!("The {}-th rarest number is: {}", n, result);
    } else {
        println!("There is no {}-th rarest number in the list.", n);
    }
}
