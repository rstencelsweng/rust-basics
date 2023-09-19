//Fibonacci Series
fn main() {
    let mut previous = 0;
    let mut current = 1;
    let mut next:isize;
    let mut count = 0;

    //Declare mutable strings to store the input in string form
    let mut num_str = String::new();
    println!("\nEnter the number of terms :");
    
    //Read the input using stdin, store the string input in num_str
    std::io::stdin().read_line(&mut num_str).unwrap();

    //Parse the input and try to convert string equivalent numbers to int
    let num:isize = num_str.trim().parse().unwrap();
    if num < 2 {
        println!("\nFibonacci series contains a minimun of two terms.\n");
    }
    else {
        print!("\n{} {} ", previous, current);
        while count < (num - 2) {
            next = previous + current;
            previous = current;
            current = next;
            print!("{} ", next);
            count = count + 1;
        }
    }
    println!("\n");
}