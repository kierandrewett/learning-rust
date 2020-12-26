fn main() {
    let s = String::from("Hello");

    takes_ownership(s);

    // println!("{}", s); // this throws an error

    let x = 5;

    make_copy(x);
}
