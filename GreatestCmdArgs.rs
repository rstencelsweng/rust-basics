//Greates from the command line Arguments
use std::env;
fn main() {
    let mut count = 1;
    let mut greatest: isize;

    //Call collect() function to fetch all the arguments
    let args:Vec<String> = env::args().collect();

    //Make sure that the user passes 2 arguments
    if args.len() < 3 {
        println!("\nPlease pass at least two numbers as command line arguments.");
    }
    else {
        let first:isize = args[1].trim().parse().unwrap();
        greatest = first;
        while count < (args.len() - 1) {
            let num:isize = args[count].trim().parse().unwrap();
            if num > greatest {
                greatest = num;
            }
            count = count +1;
        }
        println!("\nGreatest: {}\n", greatest);
    }
}