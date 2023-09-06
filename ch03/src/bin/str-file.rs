use rand::thread_rng;
use rand::Rng;
use std::fmt::Display;

trait Read {
    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}

#[derive(Debug)]
enum FileState {
    Open,
    Closed,
}

impl Display for FileState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

fn one_in(denominator: u32) -> bool {
    thread_rng().gen_ratio(1, denominator)
}

impl File {
    fn new(name: &str) -> Self {
        File {
            name: name.to_string(),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    fn new_with_data(name: &str, data: &[u8]) -> Self {
        File {
            data: data.to_owned(),
            ..File::new(name)
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl Read for File {
    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}

fn open(mut f: File) -> Result<File, String> {
    if one_in(10_000) {
        let err_msg = "Permission denied".to_string();
        return Err(err_msg);
    }
    f.state = FileState::Open;
    Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
    if one_in(10_000) {
        return Err("Interrupted by signal".to_string());
    }
    f.state = FileState::Closed;
    Ok(f)
}

fn main() {
    let f2 = File::new_with_data("2.txt", &[114, 117, 115, 116, 33]);

    let mut buffer: Vec<u8> = vec![];
    let f2 = open(f2).unwrap();
    let f2_length = f2.read(&mut buffer).unwrap();
    let f2 = close(f2).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f2);
    println!("{} is {} bytes long", f2.name, f2_length);
    println!("{}", text);

    let f3 = File::new_with_data("f3.txt", &f2.data);
    let f3 = open(f3).unwrap();
    println!("{:?}", f3);
    println!("{}", f3);
}
