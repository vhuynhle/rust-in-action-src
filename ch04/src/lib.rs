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
