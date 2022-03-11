use std::io;
use std::sync::mpsc;
use std::time::Duration;
use std::env;

mod lift;

fn helper() {
    println!("This is a lift!");
    println!("Just call him and he will come bring you were you wanna go");
    println!("Call him ? Nothing easier just type: [LOCATION]:[CALLER]:[DESTINATION]");
    println!("Example:\n\t28:thomas:5<-- will bring Thomas from the 28th floor to the 5th");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 2 && (args[1] == "-h" || args[1] == "--help") {
        helper();
        return;
    }
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
        // println!("{:?}", parsed);
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
