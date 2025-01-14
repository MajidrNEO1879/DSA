//A distinct string is a string that is present only once in an array.
//Given an array of strings arr, and an integer k, return the kth distinct string present in arr. If there are fewer than k distinct strings, return an empty string "".
use std::collections::HashMap;

pub fn kthDistinct(arr:&[&char], k:i32)-> String 
{
    let mut frequency_map: HashMap<&char, i32> = HashMap::new();

    // Count the frequency of each character
    for &c in arr {
        *frequency_map.entry(c).or_insert(0) += 1;
    }

    // Find the k-th distinct character
    let mut distinct_count = 0;
    for &&c in arr {
        if let Some(&count) = frequency_map.get(&c) {
            if count == 1 {
                distinct_count += 1;
                if distinct_count == k {
                    return c.to_string();
                }
            }
        }
    }

    // If fewer than k distinct characters, return an empty string
    "".to_string()
}