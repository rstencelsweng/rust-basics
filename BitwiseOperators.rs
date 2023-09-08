//Bitwise Operators
fn main () {
    //Declare some variables
    let x:isize = 5;
    let y:isize = 11;

    //print results of bitwise operations
    println! ("\nx = {} y = {}\n", x, y);
    println! ("\n(x | y): {}", (x | y));
    println! ("\n(x & y): {}", (x & y));
    println! ("\n(x ^ y): {}", (x ^ y));
    println! ("\n(x << 3): {}", (x << 3));
    println! ("\n(y >> 1): {}", (y >> 1));

}