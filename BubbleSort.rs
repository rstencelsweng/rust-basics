//Bubble Sort
fn main() {
    //Declare mutable string to store the input in string form
    let mut num_str = String::new();

    //Declare loop counters
    let mut i:usize = 0;
    let mut j:usize;

    //Declare temp
    let mut temp:isize;

    //Declare a mutable array of 5 inteagers, fill with zeros
    let mut num_array:[isize;5] = [0;5];

    //Loop from 0 to 4, five times
    while i < 5 {
        //Prompt the user to enter a number
        println!("\nEnter a number at index {}: ", i);

        //Clear num_str so that previous input is flushed
        num_str.clear();

        //Read the input using stdin, store the string input in num_str
        std::io::stdin().read_line(&mut num_str).unwrap();

        //Parse the input, and try to convert string equivalent numbers to int
        //Place it into an array
        num_array[i] = num_str.trim().parse().unwrap();
        //Increment i
        i += 1;
    }
    println!("\nOriginal array: {:?} ", num_array);

    //Bubble Sort
    i = 0;
    while i < num_array.len() {
        j = 0;
        while j < (num_array.len() - i - 1) {
            if num_array[j] > num_array[j + 1] {
                temp = num_array[j];
                num_array[j] = num_array[j + 1];
                num_array[j + 1] = temp;
            }
            j = j + 1;
        }
        i = i + 1;
    }
    println!("\nSorted array: {:?} ", num_array);
}