fn main() {
    let s = String::from("Hello");

    takes_ownership(s);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
