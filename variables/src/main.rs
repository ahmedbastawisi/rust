fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y = true;
    println!("The value of y is: {}", y);
    let y = false;
    println!("The value of y is: {}", y);

    const STRING: &str = "Hello, world!";
    println!("The value of STRING is: {}", STRING);
}