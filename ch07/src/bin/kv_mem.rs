use ch07::actionkv::ActionKv;

const USAGE: &str = "
Usage:
    kv_mem FILE get KEY
    kv_mem FILE delete KEY
    kv_mem FILE insert KEY VALUE
    kv_mem FILE update KEY VALUE
";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let fname = args.get(1).expect(USAGE);
    let action = args.get(2).expect(USAGE).as_ref();
    let key = args.get(3).expect(USAGE).as_ref();
    let maybe_value: Option<&String> = args.get(4);

    let path = std::path::Path::new(&fname);
    let mut store = ActionKv::open(path).expect("Unable to open file");
    store.load().expect("UNable to load data");

    match action {
        "insert" => {
            let value = maybe_value.expect(USAGE).as_ref();
            let _ = store.insert(key, value);
        }
        "update" => {
            let value = maybe_value.expect(USAGE).as_ref();
            store.update(key, value).unwrap();
        }

        "get" => match store.get(key).unwrap() {
            None => eprintln!("{:?} not found", key),
            Some(value) => println!(
                "GET: {} -> {}",
                String::from_utf8_lossy(key),
                String::from_utf8_lossy(&value)
            ),
        },
        "delete" => store.delete(key).unwrap(),
        _ => {}
    }
}
