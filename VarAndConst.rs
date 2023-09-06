//Variables Demo
fn main () {
   //Declare and initialize some variables
   let name:&str = "Willow";
   let age:usize = 36;
   let gender:char = 'F';
   let married:bool;
   let country = "Australia";
   
   //See the value of married boolean variable
   married = true;
   
   //Print evertying
   println! ("VARIABLES DEMO");
   println! ("\nName: {}\tAge: {}\tGender: {}\nMarried: {}\tCountry: {}\n", name, age, gender, married, country);
   
   //Constants Demo
   const C_NAME:&str = "Yulia";
   const C_AGE:usize = 23;
   const C_GENDER:char = 'F';
   const C_MARRIED:bool = false;
   const C_COUNTRY:&str = "Russia";
   
   //Print everything
   println! ("CONSTANTS DEMO");
   println! ("\nName: {}\tAge: {}\tGender: {}\nMarried: {}\tCountry: {}", C_NAME, C_AGE, C_GENDER, C_MARRIED, C_COUNTRY);

}
