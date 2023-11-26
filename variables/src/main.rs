fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    const CONSTANT: usize = 100_000;
    println!("The value of CONSTANT is: {}", CONSTANT);
    x = 6;
    println!("The value of x is: {}", x);

    let some_strings = "aaa";
    println!("The value of some_strings is: {}", some_strings);
    let some_strings = some_strings.len();
    println!("The value of some_strings is: {}", some_strings);

    let x = 2.0;
    let y: f32 = 3.0;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
