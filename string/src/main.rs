fn main() {
    let reference_to_nothing = dangle();
}

fn change(some_string: &mut String) {
    some_string.push_str(" world!");
}