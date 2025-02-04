#Given a string in Roman number format (s), your task is to convert it to an integer. Various symbols and their values are given below.
# Note: I = 1, V = 5, X = 10, L = 50, C = 100, D = 500, M = 1000



#Given two strings S1 and S2 as input, the task is to merge them alternatively i.e. the first character of S1 then the first character of S2 and so on till the strings end.
# NOTE: Add the whole string if other string is empty.


# Given two strings s1 and s2 consisting of lowercase characters. The task is to check whether two given strings are an anagram of each other or not.
# An anagram of a string is another string that contains the same characters, only the order of characters can be different. For example, "act" and "tac"
# are an anagram of each other. Strings s1 and s2 can only contain lowercase alphabets.
# Note: You can assume both the strings s1 & s2 are non-empty.
def anagram(str1, str2):
    if ''.join(sorted(str1)) == ''.join(sorted(str2)):
        return True
    
#Given an array arr. Find the majority element in the array. If no majority exists, return -1.
#A majority element in an array is an element that appears strictly more than arr.size()/2 times in the array.
def majority(arr):
    array_elements = {}
    m = len(arr)/2
    for i in arr:
        array_elements[i] = array_elements.get(i, 0)+1 #important
    for key in array_elements:
        if array_elements[key] > m:
            return key
#Given a sorted and rotated array arr[] of distinct elements, the task is to find the index of a target key. Return -1 if the key is not found.
def rotated_sorted(arr:list, k):
    m = len(arr)
    for i in range(m):
        if arr[i]== k:
            return i
    else:
        return -1

# Given a non-empty array arr[] of integers, find the top k elements which have the highest frequency in the array. 
# If two numbers have the same frequencies, then the larger number should be given more preference.
def top_freq(arr:list, k):
    frequency ={}
    top_freq = []
    for i in arr:
        frequency[i] = frequency.get(i, 0)+1
    for key, value in frequency.items():
        if value == k:
            top_freq.append(key)
    
    return top_freq if top_freq else -1
#Given two sorted arrays a[] and b[], find and return the median of the combined array after merging them into a single sorted array.
def median_sorted_array(arr1, arr2):
    newArray = sorted(arr1 + arr2)
    n = len(newArray)
    if n % 2 == 0:
        return (newArray[int((n/2))-1] + newArray[int((n/2))])/2
    else:
        return newArray[round(int(n/2))]
#Given an array arr[] of N elements and a number K., split the given array into K subarrays such that the maximum subarray sum achievable out of K subarrays formed is 
# minimum possible. Find that possible subarray sum.


