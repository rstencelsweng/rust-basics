//Function demo - passing array to a function
fn array_sum(num_arr:[isize;5]) -> isize {
    //Declare and initialize sum
    let mut sum = 0;
    //Loop through array
    for x in num_arr.iter() {
        sum = sum + x;
    }
    return sum;
}
fn main() {
    //Declare an integer array of b elements
    let arr = [1, 8, 5, 3, 7];
    //Pass this array to array_sum, receive sum in s
    let s = array_sum(arr);
    println!("\narr:\n {:?} \n\nSum = {}", arr, s);
}