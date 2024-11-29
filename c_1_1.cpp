#include <iostream>
#include <vector>
#include <algorithm>
#include <variant>
#include <unordered_map>
using namespace std;

// Given an array arr[] of n integers and a target value, the task is to find whether there is a
// pair of elements in the array whose sum is equal to target. This problem is a variation of 2Sum problem.

bool givenSum(vector<int> arr, int target)
{
    int len = sizeof(arr) / sizeof(arr[0]);
    // or for size : arr.size();
    for (int i = 0; i < len; i++)
    {
        for (int j = 0; j < len; j++)
        {
            if (arr[i] + arr[j] == target)
            {
                return true;
            }
        }
    }
    return false;
}
bool givenSum2(vector<int> arr, int target)
{
    sort(arr.begin(), arr.end());
    int a = 0;
    int b = arr.size() - 1;
    while (a < b)
    {
        int sum = arr[a] + arr[b];
        if (sum < target)
        {
            a++;
        }
        else if (sum > target)
        {
            b--;
        }
        else
        {
            return true;
        }
    }
    return false;
}

// Given an array of n elements that contains elements from 0 to n-1, with any of these numbers appearing any number of times.
// Find these repeating numbers in O(n) and use only constant memory space.

auto findDuplicates(vector<int> arr)
{
    unordered_map<long, int> frequency;
    int size = arr.size();
    vector<int> result = {};
    for (int i = 0; i < size; i++)
    {
        frequency[arr[i]]++;
    }
    // just to see what is in the hashmap-0-
    //  cout << "Element : Frequency\n";
    //  for (const auto& [key, value] : frequency) {
    //      cout << key << " : " << value << endl;
    //  }
    for (auto it = frequency.begin(); it != frequency.end(); ++it)
    {
        if (it->second > 1)
        {
            result.push_back(it->first);
        }
    }
    for (const auto &[key, value] : frequency)
    {
        cout << key << " : " << value << endl;
    }
}
// Given an array arr[] of n integers, construct a Product Array prod[] (of the same size) such that prod[i] is equal to the product of all the elements of arr[] except arr[i].
vector<int> productExceptSelf(vector<int> arr)
{
    int n = arr.size();
    vector<int> prod(n, 1);
    for (int i = 0; i < n; i++)
    {
        for (int j = 0; j < n; j++)
        {
            if (i != j)
            {
                prod[i] *= arr[j];
            }
        }
    }
    return prod;
}

/**Given a sorted array arr[] (may be distinct or may contain duplicates) of size N that is rotated at some unknown point,
 * the task is to find the minimum element in it.  */

int findMin(vector<int> arr)
{
    int min = arr[0];
    for (int i = 0; i < arr.size(); i++)
    {
        if (arr[i] < min)
        {
            min = arr[i];
        }
    }
    return min;
}

// Given an array arr[], the task is to find the subarray that has the maximum sum and return its sum.
vector<int> getSubarrayWithMaxSum(vector<int> arr) {
    int n = arr.size();
    vector<int> maxSubarray;  

    int maxSum = arr[0];
    maxSubarray.push_back(arr[0]); 

    for (int start = 0; start < n; start++) {
        vector<int> currentSubarray;
        int currentSum = 0;

        for (int end = start; end < n; end++) {
            currentSubarray.push_back(arr[end]);
            currentSum += arr[end];

            // Update maxSum and maxSubarray if currentSum is greater
            if (currentSum > maxSum) {
                maxSum = currentSum;
                maxSubarray = currentSubarray;
            }
        }
    }

    return maxSubarray; 
}
// all subarrays
vector<int> allSubArray(vector<int> arr)
{
    int n = arr.size();

    for (int start = 0; start < n; start++)
    {
        // Inner loop for the ending index of subarrays
        for (int end = start; end < n; end++)
        {
            // Print the subarray from `start` to `end`
            for (int k = start; k <= end; k++)
            {
                cout << arr[k] << " ";
            }
            cout << endl;
        }
    }
}

//Given a sorted and rotated array arr[] of n distinct elements, the task is to find the index of given key in the array.
//  If the key is not present in the array, return -1.

int findEl(vector<int> arr, int key)
{
    for (int i=0;i<arr.size();i++)
    {
        if(arr[i] == key)
        {
            return i;
        }
    }
    return -1;
}

/**Given n non-negative integers a1,a2,…,ana1​,a2​,…,an​ where each represents a point at coordinate (i,ai)(i,ai​). ‘ n ‘ vertical lines are drawn such that
 *  the two endpoints of line i is at (i,ai) (i,ai​) and (i,0)(i,0). Find two lines, which together with x-axis forms a container, such that the container
 *  contains the most water.
The program should return an integer which corresponds to the maximum area of water that can be contained (maximum area instead of maximum volume sounds
 weird but this is the 2D plane we are working with for simplicity). */

int waterContainer(vector<signed int> arr)
{
    int area = 0;
    for(int i=0;i<arr.size();i++)
    {
        for(int j=i+1;j<arr.size();j++)
        {
            area = max(area,min(arr[i], arr[j]) * (j-1));
        }   
    }
    return area;
}
// Find the factorial of a large number. 
long long factorial(unsigned int num)
{
    if( num == 1 || num == 0)
    {
        return 1;
    }
    else
    {
        return num * factorial(num -1);
    }
}
/**You are given an integer array score of size n, where score[i] is the score of the ith athlete in a competition. All the scores are guaranteed to be unique.

The athletes are placed based on their scores, where the 1st place athlete has the highest score, the 2nd place athlete has the 2nd highest score, and so on. The placement of each athlete determines their rank:

    The 1st place athlete's rank is "Gold Medal".
    The 2nd place athlete's rank is "Silver Medal".
    The 3rd place athlete's rank is "Bronze Medal".
    For the 4th place to the nth place athlete, their rank is their placement number (i.e., the xth place athlete's rank is "x").

Return an array answer of size n where answer[i] is the rank of the ith athlete. */
vector<string> rankS(vector<int> arr)
{
    vector <int> scores= arr;
    sort(arr.begin(), arr.end(), greater<int>());
    unordered_map<int,string> rankMap;
    for (size_t i = 0; i < scores.size(); ++i) {
        if (i == 0)
            rankMap[scores[i]] = "Gold Medal";
        else if (i == 1)
            rankMap[scores[i]] = "Silver Medal";
        else if (i == 2)
            rankMap[scores[i]] = "Bronze Medal";
        else
            rankMap[scores[i]] = std::to_string(i + 1);
    }
    vector<std::string> result;
    for (int score : scores) {
        result.push_back(rankMap[score]);
    }

    return result;
}


//Given an array arr[] of n integers where arr[i] represents the number of chocolates in ith packet. Each packet can have a variable number of chocolates. 
//There are m students, the task is to distribute chocolate packets such that: 
/**    Each student gets exactly one packet.
    The difference between the maximum and minimum number of chocolates in the packets given to the students is minimized. */
// int chocolotFactory()
// {

// }

/**Given a set of non-overlapping intervals and a new interval, the task is to insert the interval at the correct position such that after insertion, 
 * the intervals remain sorted. If the insertion results in overlapping intervals, 
 * then merge the overlapping intervals. Assume that the set of non-overlapping intervals is sorted based on start time. */

int main()
{
    vector<int> arr = {0, -1, 2, -3, 1, 2, 2, -3};
    vector<int> arr1 = {10, 3, 5, 6, 2};
    vector<int> arr2 = {2, 3, -8, 7, -1, 2, 3};
    vector<int> arr3 = {4, 5, 6, 7, 0, 1, 2};
    vector<int> rankings = {5,4,3,2,1};
    vector<std::string> ranks = rankS(rankings);

    for (const std::string& rank : ranks) {
        std::cout << rank << " ";
    }
        // int target = -2;
    // cout << givenSum(arr, target);
    // cout << givenSum2(arr, target);
    // findDuplicates(arr);
    // vector<int> result = productExceptSelf(arr1);
    // for (int x : result)
    // {
    //     cout << x << " ";
    // }
    // cout << findMin(arr1);
    // vector <int> res = getSubarrayWithMaxSum(arr2);
    // for (int i=0;i<res.size();i++)
    // {
    //     cout << res[i];
    // }
    // cout <<  findEl(arr3, 0);
    // cout << waterContainer({1,5, 4, 3});
    //cout << factorial(50); //wont work for 100;
    return 0;
}


/**Given N set of time intervals, the task is to find the intervals which don’t overlap with the given set of intervals.
Examples:  */