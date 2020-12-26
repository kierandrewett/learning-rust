fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("Length of '{}' is {}", s2, len);
}