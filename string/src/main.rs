fn main() {
    let s1 = gives_ownership();

    let s2 = String::from("me lon");

    let s3 = takes_and_gives_back(s2);

    println!("{}", s1);
    // println!("{}", s2); // cant use s2 because it's been given to s3
    println!("{}", s3);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello me lon");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
