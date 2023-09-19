//Compound Assignment Operators
fn main () {
    //Declare some variables
    let mut a:isize = -54;
    let mut b:isize = 73;
    let mut x:f64 = -3.87;
    let mut y:f64 = 4.29;

    //print everything
    println! ("\na = {} b = {}\n\nx = {} y = {}", a, b, x, y);
    a += b;
    println! ("\na = {} after performing a += b", a);
    b -= a;
    println! ("\nb = {} after performing b -= a", b);
    x *= y;
    println! ("\nx = {} after performing x *= y", x);
    y /= x;
    println! ("\ny = {} after performing y /= x", y);
    a %= b;
    println! ("\na = {} after performing a %= b", a);
    a |= b;
    println! ("\na = {} after performing a |= b", a);
    a ^= b;
    println! ("\na = {} after performing a ^= b", a);
    a <<= 3;
    println! ("\na = {} after performing a <<= 3", a);
    a &= b;
    println! ("\na = {} after performing a &= b", a);
    b >>= 2;
    println! ("\nb = {} after performing b >>= 2", b);    

}