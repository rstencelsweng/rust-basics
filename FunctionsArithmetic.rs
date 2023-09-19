//Function demo -- Function calls Arithmetic
fn sum(a: isize, b:isize) -> isize {
    //Add a and b, store in sum
    let sum = a + b;
    //Return sum to the calling funtion
    return sum;
}
fn difference(a: isize, b: isize) -> isize {
    //Subtract b from a, store in d
    let d = a - b;
    //Return d to the calling function
    return d;
}
fn product(a: isize, b: isize) -> isize {
    //Multiply a and b, store in p
    let p = a * b;
    //Return p to the calling function
    return p;
}
fn quotient(a: isize, b: isize) -> f64 {
    //Divide a and b, store in q
    let q:f64 = (a as f64) / (b as f64);
    //Return q to the calling function
    return q;
}
//Main function
fn main() {
    //Declare string to store the input in string form
    let mut num1_str = String::new();
    let mut num2_str = String::new();
    println!("\nEnter a number : ");

    //Read the input using stdin, store the string input in mun1_str
    std::io::stdin().read_line(&mut num1_str).unwrap();
    //Parse the input and try to convert string equivalent numbers to int
    //Place it into num1
    let num1:isize = num1_str.trim().parse().unwrap();
    println!("\nEnter another number :");

    //Read the input using stdin, store the string input in num2_str
    std::io::stdin().read_line(&mut num2_str).unwrap();
    //Parse the input and try to convert string equivalent numbers to int
    //Place it into num2
    let num2:isize = num2_str.trim().parse().unwrap();
    println!("\nEnter another number :");

    //Call appropriate functions
    let s = sum(num1, num2);
    let d = difference(num1, num2);
    let p = product(num1, num2);
    let q = quotient(num1, num2);

    println!("\nSum = {} \nDifference = {} \nProduct = {} \nQuotient = {}\n", s, d, p, q);
}