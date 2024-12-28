//some of the most common array and list questions

use std::collections::HashMap;

//Given an array containing n-1 numbers from 1 to n, find the missing number.
pub fn missingNumber(arr:&[i32], n: i32) ->i32
{
    let mut items_frequency: HashMap<i32, i32> = HashMap::new();
    for i in 1..=n {
        items_frequency.insert(i, 0);
    }
    for &value in arr {
        if let Some(freq) = items_frequency.get_mut(&value) {
            *freq += 1;
        }
    }
    for (key, &value) in &items_frequency {
        if value == 0 {
            return *key;
        }
    }
    -1
}   

//Merge two sorted arrays into a single sorted array without using extra space.
fn mergin(arr1:&[i32], arr2:&[i32]) ->&[i32]
{
    for i in arr2
    {
        arr1.push(i);
    }
    arr1
}