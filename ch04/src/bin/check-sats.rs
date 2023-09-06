use cubesats::StatusMessage;

fn check_status(_id: u64) -> StatusMessage {
    StatusMessage::Ok
}

fn main() {
    let sat_a = 0_u64; 
    let sat_a_status = check_status(sat_a);

    println!("{:?}: {:?}", sat_a, sat_a_status);
}
