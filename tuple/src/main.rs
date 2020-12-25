fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("{:#?}", tup);

    let (x, y, z) = tup;

    println!("What is Y?: {}\nWhat is X?: {}", y, x);
}
