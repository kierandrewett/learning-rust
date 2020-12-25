fn main() {
    let _a = [1, 2, 3, 4, 5];

    let months = [
        "January", 
        "February", 
        "March", 
        "April",
        "May", 
        "June", 
        "July", 
        "August", 
        "September", 
        "October", 
        "November", 
        "December"
    ];

    let seventh_month = months[6];

    println!("{:#?} 7th: {}", months, seventh_month);

    let invalid_month = months[56832]; // there are not 56,832 months

}
