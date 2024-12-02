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

fn main() {
    let nums: [i32; 4] = [2, 7, 11, 15];
    let target = 9;
    println!("{:?}", two_sum(&nums, target));
}
