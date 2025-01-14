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

//Write a function that reverses a string. The input string is given as an array of characters s.
pub fn reverseString(arr:&[char])->Vec<char>
{
    arr.iter().rev().cloned().collect()
}
//Given a string s, reverse only all the vowels in the string and return it.
pub fn reverseVowel(s:String) ->String
{
    let mut chars: Vec<char> = s.chars().collect(); 
    let vowels = "aeiouAEIOU"; 
    let mut p1 = 0;
    let mut p2 = chars.len() - 1;

    while p1 < p2 {
        while p1 < p2 && !vowels.contains(chars[p1]) {
            p1 += 1;
        }
        while p1 < p2 && !vowels.contains(chars[p2]) {
            p2 -= 1;
        }
        if p1 < p2 {
            chars.swap(p1, p2);
            p1 += 1;
            p2 -= 1;
        }
    }
    chars.into_iter().collect()
}