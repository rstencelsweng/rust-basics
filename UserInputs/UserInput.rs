//User Input Demo
fn main () {
    //Declare a mutable string to store the input
    let mut text = String::new();

    //Declare mutable strings to store teh input in string form
    let mut num1_str = String::new();
    let mut num2_str = String::new();
    let mut num3_str = String::new();
    let mut num4_str = String::new();

    //Prompt the user to enter something
    println! ("\nEnter some text :");

    //Read the input using stdin
    std::io::stdin().read_line(&mut text).unwrap();

    //Print whatever was read
    println! ("\nYou have entered: {}", text);

    //Prompt the user to enter a number
    println! ("\nEnter a number :");

    //Read the input using stdin, store the string input in num1_str
    std::io::stdin().read_line(&mut num1_str).unwrap();

    //Prompt the user to enter another number
    println! ("\nEnter another number :");

    //Read the input using stdin, store the string input in num2_str
    std::io::stdin().read_line(&mut num2_str).unwrap();

    //Parse the input and try to convert string equivalent numbers to int
    let num1:isize = num1_str.trim().parse().unwrap();
    let num2:isize = num2_str.trim().parse().unwrap();

    //Add both numbers
    let sum:isize = num1 + num2;

    //Print num1 num2 and sum
    println! ("\nnum1 = {} , num2 = {}  \nsum = {}\n", num1, num2, sum);

    //Promt the user to enter a floating number
    println! ("\nEnter a floating point number: ");

    //Read the input using stdin, store the string input in num3_str
    std::io::stdin().read_line(&mut num3_str).unwrap();

    //Prompt the user to enter another floating number
    println! ("\nEnter another floating point number: ");

    //Read the input using stdin, store the string input in num4_str
    std::io::stdin().read_line(&mut num4_str).unwrap();

    //Parse the input and try to convert string equivalent numbers to f64
    let num3:f64 = num3_str.trim().parse().unwrap();
    let num4:f64 = num4_str.trim().parse().unwrap();

    //Multiply both numbers
    let prod:f64 = num3 * num4;

    //Print result
    println! ("\nnum1 = {} , num2 = {} \nprod = {}\n", num3, num4, prod);
}