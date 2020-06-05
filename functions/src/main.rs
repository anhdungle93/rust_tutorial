fn main() {
    println!("Hello, world!");

    another_function();

    another_function_2(3);
}

fn another_function() {
    println!("Another function.");
}

fn another_function_2(x: i32) {
    println!("The value of x is: {}", x)
}