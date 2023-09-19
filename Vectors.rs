//Vector Update
fn main() {
    //Declare a mutable string vector using Vec::new method
    let mut names : Vec<&str> = Vec::new();

    //Declare a mutable integer vector with 3 elements
    let mut num_vector = vec![1, 0, 3];

    //Print sizes of both vectors
    println!("\nnames length: {}\nnum_vector length: {}\n", names.len(), num_vector.len());

    //Push some elements to names vector
    names.push("Igor");
    names.push("Bilal");
    names.push("Silvy");
    names.push("Bob");

    //Print both vectors and update lengths
    println!("\nnames: {:?}", names);
    println!("\nnum_vector: {:?}", num_vector);
    println!("\nnames length: {}\nnum_vector length: {}\n", names.len(), num_vector.len());

    //Add elements to num_vector
    num_vector.push(2);
    num_vector.push(5);

    println!("\nResults after final update of vectors: \n");
    println!("\nnames: {:?}", names);
    println!("\nnum_vector: {:?}", num_vector);
    println!("\nnames length: {}\nnum_vector length: {}\n", names.len(), num_vector.len());
}