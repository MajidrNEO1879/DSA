from ds_array_1 import *; from ds_string_1 import *; 
# arr_1 = [1, 2, 0, 4, 3, 0, 5, 0]
# print(zero_to_end(arr_1))
# print(three_sum([40, 20, 10, 3, 6, 7], 24))

# print (union_array([85, 25, 1, 32, 54, 6],  [85, 2] ))
# print(max_subarray([-1, -3, -10, 0, 6]))
#print(subarray_k( [10, 2, -2, -20, 10], -10))
#print(anagram('geeks', 'kseeg'))
# print(rotated_sorted([3, 5, 1, 2], 6))

# print(top_freq([1, 1, 2, 2, 3, 3, 3], 2)) 
# print(median_sorted_array([2, 3, 5, 8], [10, 12, 14, 16, 18, 20]))

from collections import Counter

def k_digit_repeating(n_str, k):
    n = len(n_str)
    freq = Counter()

    # Extract all K-digit substrings
    for i in range(n - k + 1):
        substring = n_str[i:i + k]
        freq[substring] += 1

    # Print substrings with frequency > 1
    repeating = [key for key, value in freq.items() if value > 1]
    
    return repeating

# Example usage
print(k_digit_repeating("1432543214325432", 5))