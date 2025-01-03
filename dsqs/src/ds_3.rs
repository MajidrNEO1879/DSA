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

//Two Sum - Find pairs that sum to target
fn twoSum(arr:&[i32], target:i32) ->bool
{
    // for i in 0..arr.len()
    // {
    //     for j in 0..arr.len()
    //     {
    //         if i != j && arr[i] + arr[j] ==  target
    //         {
    //             return true;
    //         }
    //     }
    // }
    // false
    for (index_i , &val_i) in arr.iter().enumerate()
    {
        for (index_j , &val_j) in arr.iter().enumerate()
        {
            if index_i != index_j && val_i + val_j == target 
            {
                return true;
            } 
        }
    }
    false
}
//Container With Most Water
/**Given n non-negative integers a1,a2,…,ana1​,a2​,…,an​ where each represents a point at coordinate (i,ai)(i,ai​). ‘ n ‘ vertical lines are drawn such that the two
 *  endpoints of line i is at (i,ai) (i,ai​) and (i,0)(i,0). Find two lines, which together with x-axis forms a container, such that the container contains the most 
 * water. */
// fn waterContainer()
// {

// }

/**Given an array arr[], the task is to find all possible indices {i, j, k} of triplet {arr[i], arr[j], arr[k]} such that their sum is equal to zero and all 
 * indices in a triplet should be distinct (i != j, j != k, k != i). We need to return indices of a triplet in sorted order, i.e., i < j < k. */
 fn threeSum(arr:&[i32]) ->Vec<(usize, usize, usize)>
 {
    let mut items = Vec::new();
    for i in 0..arr.len()
    {
        for j in 0..arr.len()
        {
            for k in 0..arr.len()
            {
                if i < j && j < k && arr[i] + arr[j] + arr[k] == 0
                {
                    items.push((i, j, k));
                }
            }
        }
    }
    items
 }
 //Given a string s, find the length of the longest substring without repeating characters
// fn longestSubString()
// {
//     let x:String = String::from("Hello world");
//     for i  in x.chars()
//     {
       
//     }
// }

/**A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, 
 * it reads the same forward and backward. Alphanumeric characters include letters and numbers. */
fn validPalindrome(phrase:String) -> bool
{
    //to remove whte spaces from all the places we can use split_whitespace.collcet()
    //but the is_alphabetic() removes that aswell.
    let mut cleaned = phrase.to_lowercase();
    cleaned.retain(|c| c.is_alphabetic());
    cleaned == cleaned.chars().rev().collect::<String>()
}
