fn main() {
    let array =vec![1,2,3,4];
   subArrays(array);
}



fn subArrays(arr:Vec<i32>) ->Vec<i32>
{
    let mut subArray:Vec<i32> = vec![];
    for i in 0..arr.len()
    {
        for j in i..arr.len()
        {
            for k in i..len()
            {
                subArray = arr[k];
            }
        }
    }
    return subArray;
}   