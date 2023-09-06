#[derive(Debug)]
pub struct CubeSat {
    pub id: u64,
    mailbox: Mailbox,
}

impl CubeSat {
    pub fn recv(&mut self) -> Option<Message> {
        self.mailbox.messages.pop()
    }
}

pub struct GroundStation;

impl GroundStation {
    pub fn send(&self, to: &mut CubeSat, msg: Message) {
        to.mailbox.messages.push(msg);
    }
}

#[derive(Debug)]
pub enum StatusMessage {
    Ok,
}

type Message = String;

#[derive(Debug)]
pub struct Mailbox {
    messages: Vec<Message>,
}

fn main() {
    let base = GroundStation {};
    let mut sat_a = CubeSat {
        id: 0,
        mailbox: Mailbox { messages: vec![] },
    };

    println!("t0: {:?}", sat_a);

    base.send(&mut sat_a, "Hello there!".to_string());
    println!("t1: {:?}", sat_a);

    let msg = sat_a.recv();
    println!("msg: {:?}", msg);
    println!("t2: {:?}", sat_a);
}
