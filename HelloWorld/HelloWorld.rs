//Hello World Program
//Main Function
fn main () {
   //println! macro to print a message on the console
   println! ("Hello World!");
   
   //println! macro to print a message on new line
   println! ("Hello");
   println! ("World!");
   
   //print! macro to print a message on same line
   print! ("Hello");
   print! ("World!");
   print! ("");
   
   //Usage of \n and \t escape sequences 
   println! ("This should be printed on the first line.\nAnd this on the next one!");
   print! ("\nLeaving another line.\tThis is what a tab-space indent looks like.\nHappy Rust!\n");
}
