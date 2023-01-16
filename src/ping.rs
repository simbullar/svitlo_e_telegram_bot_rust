use std::net::IpAddr;
use winping::{Buffer, Pinger};
use crate::telegramus::send;

pub fn ping(val: &mut &str, val2: &mut &str, val3: &mut &str, val5: &str, val6: &str) {
    let dst = std::env::args()
        .nth(1)
        .unwrap_or(String::from("162.55.27.218"))
        .parse::<IpAddr>()
        .expect("Could not parse IP Address");

    let pinger = Pinger::new().unwrap();
    let mut buffer = Buffer::new();
     
    for _ in 0..4 {
        match pinger.send(dst, &mut buffer) {
            Ok(rtt) => {
                if val2 == val3 {
                } else if val2 != val3 {
                    *val2 = ("on");
                    println!("Light is on!");
                    send(&mut val5);

                }
                *val3 = "on";
            }
            Err(err) => {
                if val2 == val3 {
                } else if val2 != val3 {
                    *val2 = ("off");
                    println!("Light is off!");
                    send(&mut val6);
                }
                *val3 = ("off");
            }
        }
    }
}