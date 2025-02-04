#You are given an array arr[] of non-negative integers. Your task is to move all the zeros in the array to the right end while maintaining the relative order of the non-zero elements. The operation must be performed in place, meaning you should not use extra space for 
# another array.
def zero_to_end(arr: list):
    n = len(arr)
    count = 0 

    for i in range(n):
        if arr[i] != 0:
            arr[count] = arr[i]
            count += 1
    
    for i in range(count, n):
        arr[i] = 0
    
    return arr

#Given an array arr[] and an integer target, determine if there exists a triplet in the array whose sum equals the given target.
#Return true if such a triplet exists, otherwise, return false.
def three_sum(arr, target):
    n = len(arr)
    
    for i in range(n - 2):
        for j in range(i + 1, n - 1):
            for k in range(j + 1, n): 
                if arr[i] + arr[j] + arr[k] == target:
                    return True
    
    return False
# Given two arrays a[] and b[], the task is to find the number of elements in the union between these two arrays.
# The Union of the two arrays can be defined as the set containing distinct elements from both arrays. If there are repetitions,
#  then only one element occurrence should be there in the union.
# Note: Elements of a[] and b[] are not necessarily distinct.
def union_array(arr1:list, arr2:list):
    return len(set(arr1 + arr2))

# Given an array arr[] that contains positive and negative integers (may contain 0 as well). Find the maximum product that we can get in a subarray of arr[].
# Note: It is guaranteed that the output fits in a 32-bit integer.
def max_subarray(arr:list):
    subArrays = []
    n = len(arr)
    for i in range(n):
        for j in range(i, n):
            subArrays.append(arr[i:j+1])  
    return max(array_prod(subArrays))
def array_prod(arr):
    n = len(arr)
    result = []
    for i in range(n): 
        prod = 1
        m = len(arr[i])
        for j in range(m): 
            prod *= arr[i][j]
        result.append(prod) 
    return result

# Given an array of integers arr[] representing a permutation, implement the next permutation that rearranges the numbers into the lexicographically next greater
# permutation. If no such permutation exists, rearrange the numbers into the lowest possible order (i.e., sorted in ascending order). 
# Note - A permutation of an array of integers refers to a specific arrangement of its elements in a sequence or linear order.


#Given an unsorted array of integers, find the number of subarrays having sum exactly equal to a given number k.
def subarray_k(arr: list, k: int) -> int:
    prefix_sum = 0
    sum_count = {0: 1}  
    sub_sum_array_count = 0

    for num in arr:
        prefix_sum += num
        sub_sum_array_count += sum_count.get(prefix_sum - k, 0)
        sum_count[prefix_sum] = sum_count.get(prefix_sum, 0) + 1

    return sub_sum_array_count
