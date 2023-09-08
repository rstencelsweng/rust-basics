//Arithmetic Operators
fn main () {
    //Declare some variables
    let a:isize = 70;
    let b:isize = 30;
    let x:f64 = 5.8;
    let y:f64 = -3.2;
    let sum:isize;
    let diff:isize;
    let prod:f64;
    let modulus:isize;
    let quo:f64;
    
    //Perform arithmetic operations
    sum = a + b;
    diff = a - b;
    modulus = a % b;
    prod = x * y;
    quo = x / y;

    //print everything Arithmetic
    println! ("ARITHMETIC OPERATORS");
    println! ("\na = {} b = {} \n\nx = {} y = {}", a, b, x, y);
    println! ("\na + b = {} \na - b = {} \na % b = {}", sum, diff, modulus);
    println! ("\nx * y = {} x / y = {}", prod, quo);
}