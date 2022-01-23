use std::io;
use std::sync::mpsc;
use std::time::Duration;

mod lift;

fn main() {
    let (transmitter, receiver) = mpsc::channel();
    let mut machine = lift::mechanic::Lift::new(
        (0..29).collect(),
        (Duration::from_secs(4), Duration::from_secs(5)),
        receiver,
    );

    std::thread::spawn(move || machine.run());
    let handler = io::stdin();
    let mut buffer = String::new();
    loop {
        handler.read_line(&mut buffer).unwrap();
        let parsed = buffer.trim().split(":").collect::<Vec<&str>>();
        match transmitter.send((
            parsed[0].parse::<isize>().unwrap(),
            parsed[1].to_string(),
            parsed[2].parse::<isize>().unwrap(),
        )) {
            Err(x) => println!("Error while communicating with lift: {}", x),
            Ok(_) => {}
        }
        buffer.clear();
    }
}
