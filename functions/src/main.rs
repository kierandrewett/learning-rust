fn main() {
    println!("Hello world");

    give_melon("Kitten");
}

fn give_melon(cat_name: &str) {
    println!("{}: 🐱🍉", cat_name);
}