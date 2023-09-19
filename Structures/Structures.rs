//Structure demo  -- returning a struct variable
//Declare Structure
struct Data {
    name:String,
    age:i32,
    gender:char,
    city:String,
    country:String
}
fn get_data() -> Data {
    //Initialize a structure variable
    let d1 = Data {
        name: String::from("Antonio"),
        age: 33,
        gender: 'M',
        city: String::from("Milan"),
        country: String::from("Italy")
    };
    return d1;
}
fn show_data(p: Data) {
    println!("\nname: {}\nage: {}\ngender: {}\ncity: {}\ncountry: {}\n", 
    p.name, p.age, p.gender, p.city, p.country);
}
fn main() {
    //Call get_data, receive structure variable
    let d = get_data();
    //Call show_data, pass d as argument
    show_data(d);
}