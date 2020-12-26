fn main() {
    let s1 = String::from("hello");

    let changed = change(&mut s1);

    println!("Length of '{}' is {}", s1, len);
}

fn change(some_string: &mut String) {
    some_string.push_str(" world!")
}