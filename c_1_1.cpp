#include <iostream>
#include <vector>
#include <algorithm>
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
       if(it->second>1)
       {
        result.push_back(it->first);
       }
    }
    for (const auto& [key, value] : frequency) {
         cout << key << " : " << value << endl;
     }
}

int main()
{
    vector<int> arr = {0, -1, 2, -3, 1, 2, 2, -3};
    // int target = -2;
    // cout << givenSum(arr, target);
    // cout << givenSum2(arr, target);
    findDuplicates(arr);
}
