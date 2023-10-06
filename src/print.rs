pub fn run() {
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
}
