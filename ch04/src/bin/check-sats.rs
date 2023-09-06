use cubesats::StatusMessage;

fn check_status(_sat_id: u64) -> StatusMessage {
    StatusMessage::Ok
}

fn main() {
    let sat_a = 0;
    let sat_a_status = check_status(sat_a);

    println!("{}: {:?}", sat_a, sat_a_status);
}
