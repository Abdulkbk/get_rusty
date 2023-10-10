use std::fmt;

#[derive(Debug)]
struct MinMax(i64, i64);

// Implement `Display` for `MinMax`
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Implement `Display` for `Point2D`
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

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

    // Pretty printing
    println!("The name is {:#?}", "Abdullahi");

    // Printing MinMax with Display
    let minmax = MinMax(0, 14);
    println!("Display: {}", minmax);

    // Printing Point2D
    let point1 = Point2D { x: 2.0, y: 4.0 };
    println!("Display: {}", point1);
    println!("Debug: {:?}", point1);

    // Printing for Complex
    let complex = Complex {
        real: 2.4,
        imag: 4.4,
    };
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
}
