//Prime or Composite
fn main() {
    //Declare mutable strings to store the input in string form
    let mut num_str = String::new();
    println!("\nEnter a number :");

    //Read the input using stdin, store the string input in num_str
    std::io::stdin().read_line(&mut num_str).unwrap();

    //Parse the input and try to convert string equivalent numbers to int
    let num:isize = num_str.trim().parse().unwrap();
    let mut i = 2;
    let mut flag:bool = false;
    while i < num {
        if num % i == 0 {
            flag = true;
            break
        }
        i += 1;
    }
    if flag {
        println!("\n{} is composite.\n", num);
    }
    else {
        println!("\n{} is prime.\n", num);
    }
}