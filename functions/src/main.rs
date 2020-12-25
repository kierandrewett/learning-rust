fn main() {
    println!("Hello world");

    give_melon("Kitten");

    let x = 5;

    let y = {
        let x = 3;
        x + 1;
    };

    println!("The value of y is: {}", y);
}

fn give_melon(cat_name: &str) {
    println!("{}: 🐱🍉", cat_name);
}