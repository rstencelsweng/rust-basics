//Basic Match Demo
fn main () {
    //Declare mutable string to store the input in string form
    let mut month_str = String::new();

    //Prompt the user to enter a number
    println! ("\nEnter the month's number (1-12) : ");

    //Read the input using stdin, store the string input in month_str
    std::io::stdin().read_line(&mut month_str).unwrap();

    //Parse the input and try to convert string equivalent numbers to int
    let month: isize = month_str.trim().parse().unwrap();

    match month {
        1 => { println! ("\nJanuary\n") },
        2 => { println! ("\nFebruary\n") },
        3 => { println! ("\nMarch\n") },
        4 => { println! ("\nApril\n") },
        5 => { println! ("\nMay\n") },
        6 => { println! ("\nJune\n") },
        7 => { println! ("\nJuly\n") },
        8 => { println! ("\nAugust\n") },
        9 => { println! ("\nSeptember\n") },
        10 => { println! ("\nOctober\n") },
        11 => { println! ("\nNovember\n") },
        12 => { println! ("\nDecember\n") },
        _ => { println! ("\nInvalid Input\n") },
    }
}