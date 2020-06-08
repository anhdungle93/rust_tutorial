fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    // the following wouldn't work because the reference s1 was dropped
    /* println!("{}, world!", s1); */
}
