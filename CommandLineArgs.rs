//Command Line arguments Demo
//Use env module from the Rust Standard Library std
use std::env;
fn sum(a:f64, b:f64) -> f64 {
    //Add a and b, store in sum
    let sum = a + b;
    //Return sum to the calling function
    return sum;
}
fn difference(a:f64, b:f64) -> f64 {
    //Subtract b from a, store in d
    let d = a - b;
    //Return d to the calling function
    return d;
}
fn product(a:f64, b:f64) -> f64 {
    //Multiply a and b, store in p
    let p = a * b;
    //Return p to the calling function
    return p;
}
fn quotient(a:f64, b:f64) -> f64 {
    //Divide a and b, store in q
    let q:f64 = a / b;
    //Return q to the calling function
    return q;
}
fn main() {
    //Call collect() function to fetch all the arguments
    let args: Vec<String> = env::args().collect();
    //Make sure that the user passes 2 arguments
    if args.len() != 3 {
        println!("\nPlease pass exactly two numbers as arguments.")
    }
    else {
        //Parse args[1] and args[2] as f64
        let num1:f64 = args[1].trim().parse().unwrap();
        let num2:f64 = args[2].trim().parse().unwrap();
        let s = sum(num1, num2);
        let d = difference(num1, num2);
        let p = product(num1, num2);
        let q = quotient(num1, num2);
        println!("\nSum = {} \nDifference = {} \nProduct = {} \nQuotient = {}\n", s, d, p, q);
    }
}
