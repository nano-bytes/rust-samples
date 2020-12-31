fn main() {
    let mut vector = Vec::new();
    vector.push(1);
    vector.push(2);
    vector.push(3);
    vector.push(4);
    vector.push(5);

    println!("Vector is: {:?}", vector);
    println!("\nVector Values:");
    for x in &vector {
        println!("{}", x);
    }
}
