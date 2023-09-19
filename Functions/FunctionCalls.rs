//Function Demo - functions calling each other
fn main() {
    println!("\nInside main function, calling function_1.");
    //Call function_1
    function_1();
    println!("\nBack inside main function, calling function_4.");
    //Call function_4
    function_4();
    println!("\nBack inside main function. Program exiting now.\n");
}
fn function_1() {
    println!("\nInside function_1, calling function_2.");
    //Call function_2
    function_2();
}
fn function_2() {
    println!("\nInside function_2, calling function_3.");
    //Call function_3
    function_3();
}
fn function_3() {
    println!("\nInside function_3, return to the calling function.");
}
fn function_4() {
    println!("\nInside function_4, calling function_2.");
    //Call function_2
    function_2();
}