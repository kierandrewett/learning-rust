fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("Length of '{}' is {}", s1, len);
}

fn calculate_length(some_string: String) -> usize {
    some_string.len()
}