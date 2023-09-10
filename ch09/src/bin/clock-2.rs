use std::net::UdpSocket;
use std::time::Duration;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use chrono::{DateTime, Timelike, TimeZone, Utc};

const NTP_MESSAGE_LENGTH: usize = 48;

/// Number of seconds between Unix epoch (1970/01/01) and
/// NTP epoch (1990/01/01)
const NTP_TO_UNIX_SECONDS: i64 = 2_208_988_800;

/// Local address used by this NTP client.
/// 12300 is the default client port for NTP.
const LOCAL_ADDR: &str = "0.0.0.0:12300";

#[derive(Default, Debug, Copy, Clone)]
struct NTPTimestamp {
    /// Number of whole seconds in the NTP timestamp
    seconds: u32,

    /// Number of fractions in the NTP timestamp
    /// 1 fraction = 2^(-32) seconds.
    fractions: u32,
}

/// NTP clock synchronization message
/// Reference:
/// Figure 8, https://datatracker.ietf.org/doc/html/rfc5905
struct NTPMessage {
    data: [u8; NTP_MESSAGE_LENGTH],
}


#[derive(Debug)]
struct NTPResult {
    /// The time the request is sent
    origin_time: DateTime<Utc>,

    /// The time the server receives the request
    receive_time: DateTime<Utc>,

    /// The time the server sends the response
    transmit_time: DateTime<Utc>,

    /// The time the client receives the response
    dst_time: DateTime<Utc>,
}

impl NTPResult {
    /// Delta value in NTP
    fn offset(&self) -> i64 {
        let duration = (self.receive_time - self.origin_time) + (self.dst_time - self.transmit_time);
        duration.num_milliseconds() / 2
    }

    /// Theta value in NTP
    fn delay(&self) -> i64 {
        let duration = (self.dst_time - self.origin_time) - (self.transmit_time - self.receive_time);
        duration.num_milliseconds()
    }
}

impl From<&NTPTimestamp> for DateTime<Utc> {
    fn from(ntp: &NTPTimestamp) -> Self {
        let secs = ntp.seconds as i64 - NTP_TO_UNIX_SECONDS;
        let nanos = (ntp.fractions as f64) * 1E9 / 2_f64.powi(32);
        Utc.timestamp_nanos(secs * 1_000_000_000 + nanos as i64)
    }
}

impl From<&DateTime<Utc>> for NTPTimestamp {
    fn from(utc: &DateTime<Utc>) -> Self {
        let secs = utc.timestamp() + NTP_TO_UNIX_SECONDS;
        let fraction = (utc.nanosecond() as f64) / 1E9 * 2_f64.powi(32);

        NTPTimestamp {
            seconds: secs as u32,
            fractions: fraction as u32,
        }
    }
}

impl NTPMessage {
    fn new() -> Self {
        NTPMessage {
            data: [0; NTP_MESSAGE_LENGTH]
        }
    }

    /// Generate a client-side message
    fn client(utc: &DateTime<Utc>) -> Self {
        const LI_NO_WARNING: u8 = 0b00;
        const VERSION: u8 = 0b011;
        const MODE_CLIENT: u8 = 0b011;

        let mut msg = NTPMessage::new();
        msg.data[0] = (LI_NO_WARNING << 6) | (VERSION << 3) | MODE_CLIENT;
        let origin_timestamp = NTPTimestamp::from(utc);
        let mut origin_timestamp_writer: &mut [u8] = &mut msg.data[24..32];
        origin_timestamp_writer.write_u32::<BigEndian>(origin_timestamp.seconds).unwrap();
        origin_timestamp_writer.write_u32::<BigEndian>(origin_timestamp.fractions).unwrap();

        msg
    }

    /// Parse the timestamp starting from byte #i.
    fn parse_timestamp(&self, i: usize) -> std::io::Result<NTPTimestamp> {
        let mut reader = &self.data[i..i + 8];
        let seconds = reader.read_u32::<BigEndian>()?;
        let fractions = reader.read_u32::<BigEndian>()?;

        Ok(NTPTimestamp {
            seconds,
            fractions,
        })
    }

    fn rx_time(&self) -> std::io::Result<NTPTimestamp> {
        self.parse_timestamp(32)
    }

    fn tx_time(&self) -> std::io::Result<NTPTimestamp> {
        self.parse_timestamp(40)
    }
}

fn weighted_mean(values: &[f64], weights: &[f64]) -> f64 {
    let mut result = 0.0;
    let mut sum_of_weights = 0.0;

    for (v, w) in values.iter().zip(weights) {
        result += v * w;
        sum_of_weights += w;
    }

    result / sum_of_weights
}


fn ntp_round_trip(
    host: &str,
    port: u16,
) -> std::io::Result<NTPResult> {
    let destination = format!("{}:{}", host, port);
    let timeout = Duration::from_secs(1);

    let udp = UdpSocket::bind(LOCAL_ADDR)?;
    udp.connect(destination).expect("Unable to connect");

    // Create and send the request
    let t1 = Utc::now();
    let request = NTPMessage::client(&t1);
    udp.send(&request.data)?;
    udp.set_read_timeout(Some(timeout))?;

    // Get the response
    let mut response = NTPMessage::new();
    udp.recv_from(&mut response.data)?;

    // The time we receive the response from the server
    let t4 = Utc::now();
    let t2 = DateTime::<Utc>::from(&response.rx_time()?);
    let t3 = DateTime::<Utc>::from(&response.tx_time()?);

    Ok(NTPResult {
        origin_time: t1,
        receive_time: t2,
        transmit_time: t3,
        dst_time: t4,
    })
}

fn check_time() -> std::io::Result<f64> {
    const NTP_PORT: u16 = 123;
    let servers = [
        "time.nist.gov",
        "time.euro.apple.com",
        "time.apple.com",
    ];
    let mut times = Vec::with_capacity(servers.len());
    for &server in servers.iter() {
        print!("{} => ", server);
        let calc = ntp_round_trip(server, NTP_PORT);
        match calc {
            Ok(time) => {
                println!(" {}ms away from local system time", time.offset());
                times.push(time);
            }
            Err(e) => {
                eprintln!(" ? [{}]", e);
            }
        };
    }

    let mut offsets = Vec::with_capacity(servers.len());
    let mut offset_weights = Vec::with_capacity(servers.len());

    for time in times {
        let offset = time.offset() as f64;
        let delay = time.delay() as f64;
        let weight: f64 = 1_000_000.0 / (delay * delay);
        if weight.is_finite() {
            offsets.push(offset);
            offset_weights.push(weight);
        }
    }

    let avg_offsets = weighted_mean(&offsets, &offset_weights);
    Ok(avg_offsets)
}

fn main() {
    let offset = check_time().unwrap() as isize;
    let adjust_ms = offset.signum() * offset.abs().min(200) / 5;
    let adjust_ms = chrono::Duration::milliseconds(adjust_ms as i64);
    let now: DateTime<Utc> = Utc::now() + adjust_ms;

    println!("Local time: {}", now.to_rfc3339());
    println!("Adjustment: {} ms", adjust_ms);
}