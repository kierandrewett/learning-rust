fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("{:#?}", tup);

    let (x, y, z) = tup;

    println!("What is Y?: {}\nWhat is X?: {}\nWhat is Z?: {}", y, x, z);

    let tup2: (i32, f64, u8) = (500, 6.4, 1);
    
    let five_hundred = tup2.0;

    let six_point_four = tup2.1;

    let one = tup2.2;

    println!("{} {} {}", five_hundred, six_point_four, one);
}
