fn main() {
    let s1 = String::from("hello");

    let changed = change(&mut s1);

    println!("{} = {}", s1, changed);
}

fn change(some_string: &mut String) -> &mut String {
    some_string.push_str(" world!");

    some_string
}