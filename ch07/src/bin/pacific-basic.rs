use std::collections::HashMap;

fn main() {
    let mut capitals = HashMap::new();

    capitals.insert("Cook Island", "Avarua");
    capitals.insert("Fiji", "Suva");
    capitals.insert("Kiribati", "South Tarawa");
    capitals.insert("Niue", "Alofi");
    capitals.insert("Tonga", "Nuku'alofa");
    capitals.insert("Tuvalu", "Funafuti");

    let tonga_capital = capitals["Tonga"];
    println!("The capital of Tonga: {}", tonga_capital);
}
