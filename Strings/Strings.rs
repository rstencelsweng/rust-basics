//String Demo --Split/Tokenize
fn main() {
    let str1 = String::from("Russia is the largest country by area");
    println!("\nstr1: {}", str1);

    let str2 = String::from("Netherlands,Norway,Japan<USA,India,Singapore");
    println!("\nstr2: {}", str2);

    //Tokenize str1 using _whitespace
    println!("\nTokenized str1 by whitespace:\n");
    for str_token in str1.split_whitespace() {
        println!("{}", str_token);
    }

    //Tokenize str2 using split(",")
    println!("\nTokenized str2 by comma:\n");
    for str_token in str2.split(",") {
        println!("{}", str_token);
    }

}