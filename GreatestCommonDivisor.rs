//GCD
fn main() {
    //Declare mutable strings to store the input in string form
    let mut num1_str = String::new();
    let mut num2_str = String::new();

    //Prompt the user to enter a number
    println!("\nEnter a number :");

    //Read the input using stdin, store the string input in num1_str
    std::io::stdin().read_line(&mut num1_str).unwrap();

    //Prompt the user to enter another number
    println!("\nEnter another number :");

    //Read the input using stdin, store the string input in num2_str
    std::io::stdin().read_line(&mut num2_str).unwrap();

    //Parse the input and try to convert string equivalent numbers to int
    let num1:isize = num1_str.trim().parse().unwrap();
    let num2:isize = num2_str.trim().parse().unwrap();
    let mut numerator:isize;
    let mut denominator:isize;
    let mut remainder:isize;
    let gcd:isize;
    if num1 > num2 {
        numerator = num1;
        denominator = num2;
    }
    else {
        numerator = num2;
        denominator = num1;
    }
    remainder = numerator % denominator;
    while remainder != 0 {
        numerator = denominator;
        denominator = remainder;
        remainder = numerator % denominator;
    }
    gcd = denominator;
    println!("\nGCD of {} and {} is {}\n", num1, num2, gcd);
}