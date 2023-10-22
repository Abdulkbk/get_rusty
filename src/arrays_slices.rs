use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

pub fn run() {
    // Fixed array
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // Indexing
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    // Number of elements
    println!("Number of elements: {}", xs.len());

    // Arrays are stacked allocated
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    // Borrow the whole array as slice
    analyze_slice(&xs);

    // Borrow a segment of the array
    analyze_slice(&xs[1..3]);

    // Accessing arrays using `.get`
    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }
}
