//Reverse an array
fn main() {
    //Declare mutable string to store the input in string form
    let mut num_str = String::new();

    //Declare a loop counter
    let mut i = 0;

    //Declare temp
    let mut temp:isize;

    //Declare a mutable array of 5 integers, fill with zeros
    let mut num_array:[isize;5] = [0;5];

    //Loop from 0 to 4, five times
    while i < 5 {
        //Prompt the user to enter a number
        println!("\nEnter a number at index {}: ", i);

        //Clear num_str so that previous input is flushed
        num_str.clear();
        //Read the input using stdin, store the string input in num_str
        std::io::stdin().read_line(&mut num_str).unwrap();

        //Parse teh input and try to convert string equivalent numbers to int
        //Place it into an array
        num_array[i] = num_str.trim().parse().unwrap();
    //Increment
    i += 1;
    }
    println!("\nOriginal arra: {:?} ", num_array);
    i = 0;
    while i < num_array.len()/2 {
        temp = num_array[i];
        num_array[i] = num_array[num_array.len() - 1 - i];
        num_array[num_array.len() - 1 - i] = temp;
        i += 1;
    }
    println!("\nReversed array: {:?} ", num_array);
}