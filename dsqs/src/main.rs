use core::num;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
mod ds_2;
mod ds_3;
mod top_50_2;
mod linked_list_q;
mod tree_r;
mod string_qs;
mod matrix;
use tree_r::TreeNode;

/**Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
You may assume that each input would have exactly one solution, and you may not use the same element twice.
You can return the answer in any order. */
fn two_sum(arr: &[i32], target: i32) -> Vec<i32> {
    for i in 0..arr.len() {
        for j in (i + 1)..arr.len() {
            if arr[i] + arr[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![]
}
//removing duplicate using sets
fn testing(arr: [i32; 5]) -> HashSet<i32> {
    let set: HashSet<i32> = arr.into_iter().collect();
    return set;
}
/**Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order. */
fn search_index(arr: &[i32], target: i32) -> i32 {
    for i in 0..arr.len() {
        if arr[i] == target {
            return i as i32;
        }
    }
    -1
}

/**Given a non-empty array of integers nums, every element appears twice except for one. Find that single one.
You must implement a solution with a linear runtime complexity and use only constant extra space. */

fn singleNumber(arr: &[i32]) -> i32 {
    let mut freq_map = HashMap::new();
    for &num in arr {
        *freq_map.entry(num).or_insert(0) += 1;
    }
    for (&key, &value) in &freq_map {
        if value == 1 {
            return key;
        }
    }
    -1
}
fn count_items(arr: &[i32]) {
    let mut freq_map = HashMap::new();

    // Populate the frequency map
    for &num in arr {
        *freq_map.entry(num).or_insert(0) += 1;
    }

    // Print the frequencies
    println!("Item frequencies:");
    for (key, value) in &freq_map {
        println!("{}: {}", key, value);
    }
}

//Given a 2D integer array matrix, return the transpose of matrix.
// The transpose of a matrix is the matrix flipped over its main diagonal, switching the matrix's row and column indices.
fn transpose(mut arr: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    for i in 0..arr.len() {
        for j in i + 1..arr.len() {
            let temp = arr[i][j];
            arr[i][j] = arr[j][i];
            arr[j][i] = temp;
        }
    }
    return arr;
}

//Given a boolean matrix mat[M][N] of size M X N, modify it such that if a matrix cell mat[i][j] is 1 (or true) then make all the cells of ith row and jth
//column as 1.

/**you are given a sorted unique integer array nums.
A range [a,b] is the set of all integers from a to b (inclusive).
Return the smallest sorted list of ranges that cover all the numbers in the array exactly. That is, each element of nums is covered by exactly one of the ranges,
 and there is no integer x such that x is in one of the ranges but not in nums.
Each range [a,b] in the list should be output as:
    "a->b" if a != b
    "a" if a == b
 */

/**Given an array of integers nums, half of the integers in nums are odd, and the other half are even.
Sort the array so that whenever nums[i] is odd, i is odd, and whenever nums[i] is even, i is even.
Return any answer array that satisfies this condition. */
fn oddEvenIndex(arr: &[i32]) -> Vec<i32> {
    let mut evens = vec![];
    let mut odds = vec![];
    for i in 0..arr.len() {
        if arr[i] % 2 == 0 {
            odds.push(arr[i]);
        } else {
            evens.push(arr[i]);
        }
    }
    //creating an array of len arr with elements as 0
    let mut result = vec![0; arr.len()];
    // print!("{:?}", result);
    let mut even_index = 0;
    let mut odd_index = 1;
    for i in 0..arr.len() / 2 {
        result[even_index] = evens.pop().unwrap();
        result[odd_index] = odds.pop().unwrap();
        even_index += 2;
        odd_index += 2;
    }
    return result;
}
// Given an integer array, the task is to find the maximum product of any subarray.
fn maxSubArray(arr: Vec<i32>) ->Option<i32>
{
    //creating all subarrays
    let mut subArray: Vec<Vec<i32>> = Vec::new();
    for i in 0..arr.len() {
        for j in i..arr.len() {
            subArray.push(arr[i..=j].to_vec());
        }
    }
    let mut maxEl: Vec<i32> = Vec::new();

    for i in subArray {
        let mut max = 1;
        for j in i {
            max *= j;
        }
        maxEl.push(max);
    }
    maxEl.iter().max().cloned()
}
//Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value to go outside the signed 32-bit integer
//  range [-231, 231 - 1], then return 0. Assume the environment does not allow you to store 64-bit integers (signed or unsigned).
fn reverse (num: i32)
{
     let y =num.to_string()
    .chars()
    .rev()
    .collect::<String>()
    .parse::<i32>()
    .unwrap();
    //a better more clear solution
    //et num_string = num.to_string();
    //let reversed_string: String = num_string.chars().rev().collect();
    //reversed_string.parse()
}
//Given an integer x, return true if x is a palindrome, and false otherwise.
fn palindrome(num: i32) ->bool
{
    let y =num.to_string()
    .chars()
    .rev()
    .collect::<String>()
    .parse::<i32>()
    .unwrap();
   if y == num
   {
    true
   }
   else {
       false
   }
}   
/**You are climbing a staircase. It takes n steps to reach the top.
Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top? */
fn steps (number:i32) ->i32
{
    if number ==1 || number == 0
    {
        return 1;
    }
    else
    {
        return steps(number -1) + steps( number -2);
    }
}
fn isValid(s: String)->bool
{
    let mut stack: Vec<char> = Vec::new();

    for ch in s.chars() {
        match ch {
            '(' | '{' | '[' => stack.push(ch), 
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            _ => return false, 
        }
    }

    stack.is_empty() 
}

//Given two integer arrays nums1 and nums2, return an array of their intersection. Each element in the result must appear as many 
//times as it shows in both arrays and you may return the result in any order.
fn intersect(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let mut intersect: Vec<i32> = Vec::new();
    let mut arr2 = arr2; 

    for i in &arr1 {
        let mut found = false;
        for j in 0..arr2.len() {
            if *i == arr2[j] {
                intersect.push(*i); 
                arr2.remove(j); 
                found = true;
                break; 
            }
        }
        if found {
            continue; 
        }
    }

    intersect
}
/**Given an array points where points[i] = [xi, yi] represents a point on the X-Y plane, return true if these points are a boomerang. */

fn main() {
    let nums: [i32; 4] = [2, 7, 11, 15];
    let num1 = [1, 4, 5, 5, 6];
    let arr2 = [1, 3, 5, 6];
    let array = vec![-1, -3, -10, 0, 60];
    let evenOdd = [4, 2, 5, 7];
    let result: HashSet<i32> = testing(num1);
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let transposed = transpose(matrix);
    let num:i32 = 5;
    // let y = reverse(num);
    //let palindoreEx = palindrome(num);
    let steps = steps(num);
    //print!("{}", steps); 
    // println!("{:?}", two_sum(&nums, target));
    //let nums = vec![4, 1, 2, 1, 2, 4, 4];
    // count_items(&nums);
    // println!("{:?}", result);
    //print!("{}", search_index(&arr2, 5));
    // oddEvenIndex(&evenOdd);
    // println!("{:?}", transposed);
    // print!("{:?}", maxSubArray(array));
    let input = String::from("swiss");
    // println!("Is valid: {}", isValid(input));
    // let array = vec![4,3,2,1];
    // let result = ds_2::plus_one(array);
    // println!("Result: {:?}", result); 
    //let items1 = vec![1,2,2,1];
   
    //let result = ds_2::mergedLists(items1, items2);
    //println!("{:?}", result);
    //println!("{:?}", intersect(items1, items2));
    // let item2 = vec![1, 2, 4, 6, 3, 7, 8];
    // let n = 8;
    // let missing = ds_3::missingNumber(&item2, n);
    // println!("The missing number is: {}", missing);
    //linked_list_q::reverseLinkedList();
    //let waterheight: [i32; 8] = [2, 1, 8, 6, 4, 6, 5, 5];
    //let y = ds_3::waterContainer(&waterheight);
    // match ds_3::non_repeat(input) {
    //     Some(ch) => println!("First non-repeating character: {}", ch),
    //     None => println!("No non-repeating character found"),
    // }
    // let tree = TreeNode {
    //     value: 1,
    //     left: Some(Box::new(TreeNode {
    //         value: 2,
    //         left: Some(Box::new(TreeNode {
    //             value: 4,
    //             left: None,
    //             right: None,
    //         })),
    //         right: Some(Box::new(TreeNode {
    //             value: 5,
    //             left: None,
    //             right: None,
    //         })),
    //     })),
    //     right: Some(Box::new(TreeNode {
    //         value: 3,
    //         left: None,
    //         right: Some(Box::new(TreeNode {
    //             value: 6,
    //             left: None,
    //             right: None,
    //         })),
    //     })),
    // };

    // // Calculate the size of the tree
    // let size = tree.size();
    // println!("The size of the tree is: {}", size);
    let characters = [&'a', &'b', &'a', &'c', &'b', &'d'];
    let k = 2;
    let result = string_qs::kthDistinct(&characters, k);
    // println!("{}", result); 
    let chars = ['a', 'b', 'c','d'];
    // print!("{:?}", string_qs::reverseString(&chars));
    let input:String = String::from("leetcode");
    let result =string_qs::reverseVowel(input);
    // println!("{}", result); 
    let grid = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];
    let k = 1;
    let shifted_grid = matrix::shift_grid(grid, k);
    println!("{:?}", shifted_grid);
}
