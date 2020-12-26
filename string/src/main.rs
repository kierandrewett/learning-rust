fn main() {
    let s1 = gives_ownership();

    let s2 = String::from("me lon");

    let s3 = takes_and_gives_back(s2);
}
