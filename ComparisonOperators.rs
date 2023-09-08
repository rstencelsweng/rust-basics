//Comparison Operators
fn main () {
    //Declare some variables
    let a = 10;
    let b = 60;
    let c:f64;
    let d:f64;
    c = 7.89;
    d = -4.42;

    //print a b c d
    println! ("\na = {} b = {} c = {} d = {}", a, b, c,d);

    //print results of comparisons
    println! ("\n\na > b: {}", (a > b));
    println! ("\nd < c: {}", (d < c));
    println! ("\na == b: {}", (a == b));
    println! ("\nc != d: {}", (c != d));
    println! ("\na <= b: {}", (a <= b));
    println! ("\nd >= c: {}", (d >= c));

}