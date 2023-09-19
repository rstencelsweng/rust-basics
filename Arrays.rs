//Array sum and average
fn main() {
    //Declare mutable string to store the input in string form
    let mut num_str = String::new();

    //Declare a loop counter
    let mut i = 0;

    //Declare sum and avg
    let mut sum:isize = 0;
    let avg:f64;

    //Declare a mutable array of 5 integers, fill with zeros
    let mut num_array: [isize;5] = [0;5];

    //Loop from 0 to 4, fives times
    while i < 5 {
        //Prompt the user to enter a number
        println! ("\nEnter a number at index {}: ",i);

        //Clear the num_str so that previous input is cleared
        num_str.clear();

        //Read the input using stdin, store the string input in num_str
        std::io::stdin().read_line(&mut num_str).unwrap();

        //Parse the input and try to convert string equivalent numbers to int
        //Place it into an array
        num_array[i] = num_str.trim().parse().unwrap();

        //Add current element to sum
        sum = sum + num_array[i];

        //Incremement i 
        i += 1;
    }
    //Calculate average
    avg = (sum as f64) / 5.0;

    //Print Everything
    println!("\nnum_array: {:?}", num_array);
    println!("\nnum_array Size: {}", num_array.len());
    println!("\nSum of Elements: {}", sum);
    println!("\nAverage: {:?}", avg);
}