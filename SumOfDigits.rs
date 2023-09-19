//Sum of Digits
fn main() {
    let mut sum:isize = 0;

    //Declare mutable strings to store the input in string form
    let mut num_str = String::new();
    println!("\nEnter a number :");

    //Read the input using stdin, store the string input in num_str
    std::io::stdin().read_line(&mut num_str).unwrap();

    //Parse the input and try to convert string equivalent numbers to int
    let mut num:isize = num_str.trim().parse().unwrap();
    while num > 0 {
        sum = sum + (num % 10);
        num = num / 10;
    }
    println!("\nSum = {}\n", sum);
}