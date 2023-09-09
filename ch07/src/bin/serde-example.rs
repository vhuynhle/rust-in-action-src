use serde::Serialize;

#[derive(Serialize, Debug)]
struct City {
    name: String,
    population: usize,
    latitude: f64,
    longitude: f64,
}

fn main() {
    let calabar = City {
        name: "Calabar".to_string(),
        population: 470_000,
        latitude: 4.95,
        longitude: 8.33,
    };

    println!("Debug: {:?}", calabar);

    let as_json = serde_json::to_string(&calabar).unwrap();
    println!("Json: {}, size: {}", as_json, as_json.as_bytes().len());

    let as_bincode = bincode::serialize(&calabar).unwrap();
    println!("Bincode: {:02X?}, size: {}", as_bincode, as_bincode.len());
    println!(
        "Bincode (as UTF-8): {}",
        String::from_utf8_lossy(&as_bincode)
    );

    let as_cbor = serde_cbor::to_vec(&calabar).unwrap();
    println!("Cbor: {:02X?}, size: {}", as_cbor, as_cbor.len());
    println!("Cbor (as UTF-8): {}", String::from_utf8_lossy(&as_cbor));
}
