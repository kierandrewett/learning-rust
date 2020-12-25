fn main() {
    println!("Hello world");

    give_melon("Kitten");

    let _x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let number_five = five();

    println!("{}", number_five);
}

fn five() -> i32 {
    5
}

fn give_melon(cat_name: &str) {
    println!("{}: 🐱🍉", cat_name);
}