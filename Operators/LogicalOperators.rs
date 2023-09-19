//Logical Operators
fn main () {
    //Declare some variables
    let x:isize = 23;
    let y:isize = 44;
    const Z:isize = 23;

    //print results of random comparisons
    println! ("\nx = {} y = {} Z = {}\n", x, y, Z);
    println! ("\n(x == y): {}", (x == y));
    println! ("\n(x == Z): {}", (x == Z));
    println! ("\n(x < y): {}", (x < Z));
    println! ("\n(Z > x): {}", (Z > x));
    println! ("\n\n(x == y) || (Z != y): {}", ((x == Z) || (Z != y)));
    println! ("\n(x == Z) && (x > y): {}", ((x == Z) && (x > y)));
    println! ("\n!(Z > x): {}", !(Z > x));
}