use std::fmt;
// Tuples can be used as fn args or return values
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // Let can be used to bind the members of a tuple to variables
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}, {}, {}, {})",
            self.0, self.1, self.2, self.3, self.4
        )
    }
}

pub fn run() {
    let a = reverse((12, false));

    println!("{:?}", a);
    println!("{} and {}", a.0, a.1);

    // You cannot print long tuples (13 el and above)
    let long_tuple = (
        1, 2, 3, true, true, false, false, "wow", "moon", "tool", "lol", "Works",
    );
    println!("Long tuple: {:?}", long_tuple);

    let mat_numbers = Matrix(1.2, 2.2, 3.1, 4.5, 5.5);
    println!("Matrix: {:?}", mat_numbers);
    println!("Matrix: {}", mat_numbers);
}
