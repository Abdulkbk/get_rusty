pub fn run() {
    /*
    This is a comment
    that spans through multiple
    lines
     */
    println!("Hello from Rust");

    // Printing a Number
    println!("Number: {}", 1.5);

    // Printing named arguments
    println!(
        "My name is {name} and I am {age}",
        name = "Abdullahi",
        age = 22
    );

    // Printing Positional arguments
    println!(
        "I am using {0} and a {0} Laptop. They are based in {1}.",
        "Huawei", "China"
    );

    // Formatting
    println!("Base 10: {}", 100);
    println!("Base 2: {:b}", 100);
    println!("Base 8: {:o}", 100);
    println!("Base 16: {:x}", 10);

    // Right justify (give x-1 space before value)
    println!("{number:>5}", number = 6);

    // Named args for width
    println!("{number:>width$}", number = 6, width = 6);

    // Padding numbers with extra x before the value
    println!("{number:1>5}", number = 6);

    // Print floating numbers with precision
    let pi = 3.141592;
    println!("PI: {:.3}", pi);
    println!("PI: {1:.0$}", 3, pi);
    println!("PI: {:.*}", 3, pi);
}
