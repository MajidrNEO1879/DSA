pub fn merginArrays(arr1: &mut Vec<i32> , arr2:&[i32])
{
    for (i , &value) in arr2.iter().enumerate()
    {
        arr1.push(value);
    }
    arr1.sort();
}

//Given an integer array nums, find the subarray with the largest sum, and return its sum.
pub fn max_sub_array(arr:&[i32])->i32
{
    let n = arr.len();
    let mut max_sum = arr[0];
    for i in 0..n {
        for j in i..n {
            let mut current_sum = 0;
            // Calculate the sum of the subarray arr[i..j+1]
            for k in i..=j {
                current_sum += arr[k];
            }
            // Update max_sum if the current subarray sum is greater
            max_sum = max_sum.max(current_sum);
        }
    }
    max_sum 
}
//Given an array arr[] of n integers, construct a Product Array prod[] (of the same size) 
//such that prod[i] is equal to the product of all the elements of arr[] except arr[i]. 

pub fn same_array(arr:&[i32]) ->Vec<i32>
{
    let n = arr.len();
    if n == 0 {
        return vec![];
    }
//Prefix: The product of all elements before a given index in the array.
//Suffix: The product of all elements after a given index in the array.
    let mut prod = vec![1; n];
    let mut prefix = 1;
    for i in 0..n {
        prod[i] = prefix;
        prefix *= arr[i];
    }
    let mut suffix = 1;
    for i in (0..n).rev() {
        prod[i] *= suffix;
        suffix *= arr[i];
    }
    prod
}