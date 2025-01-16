use std::vec;

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut digits = digits; 
    let mut carry = 1;       

    for i in (0..digits.len()).rev() {
        let sum = digits[i] + carry;
        digits[i] = sum % 10;
        carry = sum / 10;     

        if carry == 0 {
            break; 
        }
    }

    if carry > 0 {
        digits.insert(0, carry); 
    }

    digits
}
//You are given the heads of two sorted linked lists list1 and list2.
// Merge the two lists into one sorted list. The list should be made by splicing together the nodes of the first two lists.
// Return the head of the merged linked list.
pub fn mergedLists(arr1 : Vec<i32>, arr2 :Vec<i32>)->Vec<i32>
{
    let mut merged: Vec<i32> = Vec::new(); 

    let min_len = arr1.len().min(arr2.len()); 

    for i in 0..min_len {
       
        merged.push(arr1[i]);
        merged.push(arr2[i]);
    }
    if arr1.len() > min_len {
        merged.extend_from_slice(&arr1[min_len..]);
    } else if arr2.len() > min_len {
        merged.extend_from_slice(&arr2[min_len..]);
    }
    merged
   
}

//write a recursive function to count the items in a list
fn arrayLen(arr:&Vec<i32>)->usize
{
   if arr.is_empty()
   {
    return 0;
   }
   return 1 + arrayLen(&arr[1..].to_vec());
}
//all the sub arrays

pub fn subArrays (arr:&[i32])->Vec<Vec<i32>>
{
    let mut subarrays = Vec::new();

    for start in 0..arr.len() {
        for end in start + 1..=arr.len() {
            subarrays.push(arr[start..end].to_vec());
        }
    }

    subarrays
}

//from all subarrays what is the maximum and minimum one?
pub fn max_subarray(arr: &[i32]) -> i32 {
    let sub_arrays = subArrays(arr);
    let mut max_sum = i32::MIN; 

    for subarray in sub_arrays {
        let sum: i32 = subarray.iter().sum(); 
        if sum > max_sum {
            max_sum = sum; 
        }
    }

    max_sum
}