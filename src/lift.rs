pub mod mechanic {
    use std::collections::HashMap;
    use std::sync::mpsc;
    use std::thread;
    use std::time::{Duration, Instant};

    #[derive(Debug)]
    pub struct Person {
        name: String,
        floor: isize,
    }

    #[derive(Debug)]
    enum Direction {
        Up,
        Down,
        Stable,
    }

    #[derive(Debug)]
    pub struct Lift {
        inactivity: bool,
        inactivity_time: Duration,
        basement: isize,
        requested: HashMap<isize, Vec<Person>>,
        floors: Vec<isize>,
        climbing_speed: Duration,
        descent_speed: Duration,
        current_floor: isize,
        order: HashMap<isize, Vec<Person>>,
        trajectory: Vec<isize>,
        direction: Direction,
        channel_receiver: mpsc::Receiver<(isize, String, isize)>,
    }

    impl Lift {
        pub fn new(
            floors: Vec<isize>,
            speed: (Duration, Duration),
            rx: mpsc::Receiver<(isize, String, isize)>,
        ) -> Lift {
            let basement = floors[0];
            Lift {
                inactivity: true,
                inactivity_time: Duration::new(0, 0),
                basement,
                floors,
                requested: HashMap::new(),
                climbing_speed: speed.0,
                descent_speed: speed.1,
                current_floor: basement,
                order: HashMap::new(),
                trajectory: Vec::new(),
                direction: Direction::Stable,
                channel_receiver: rx,
            }
        }

        fn go_next_floor(&mut self) -> () {
            if self.trajectory.is_empty() {
                return;
            }
            while !self.floors.contains(&self.trajectory[0]) {
                println!("Unavailable floor: {}", self.trajectory[0]);
                self.trajectory.swap_remove(0);
            }
            let target = self.trajectory[0];
            thread::sleep(if target > self.current_floor {
                self.climbing_speed
            } else {
                self.descent_speed
            });
            self.current_floor = target;
            self.trajectory.swap_remove(0);
        }

        fn empty_passenger(&mut self) -> () {
            println!("Floor {}", self.current_floor);
            for i in self.order.entry(self.current_floor).or_default() {
                println!("Passenger {} you can get down. Have a nice day!", i.name);
            }
            let to = self.order.entry(self.current_floor);
            to.or_default();
        }

        fn get_passenger(&mut self) -> () {
            let passengers = self.requested.remove(&self.current_floor);
            if let Some(pass) = passengers {
                for i in pass {
                    let destination = i.floor;
                    self.order.entry(i.floor).or_default().push(i);
                    if !self.trajectory.contains(&destination) {
                        self.trajectory.push(destination);
                    }
                }
            }
        }

        fn get_request(&mut self) -> () {
            loop {
                let person = self.channel_receiver.try_recv();
                match person {
                    Ok((floor, name, dest)) => {
                        let p = Person { name, floor: dest };
                        self.requested.entry(floor).or_default().push(p);
                        if !self.trajectory.contains(&floor) {
                            self.trajectory.push(floor);
                        }
                        self.inactivity = false;
                        // self.inactivity_time = time
                    }
                    Err(_) => return,
                }
            }
        }

        fn create_trajectory(&mut self) -> () {
            let mut prevs = self
                .trajectory
                .iter()
                .filter(|&e| *e < self.current_floor)
                .collect::<Vec<_>>();
            let mut nexts = self
                .trajectory
                .iter()
                .filter(|&e| *e > self.current_floor)
                .collect::<Vec<_>>();

            prevs.sort_by(|a, b| b.cmp(a));
            nexts.sort();
            if let Direction::Up = self.direction {
                nexts.append(&mut prevs);
                let mut new_trajectory = Vec::new();
                for i in nexts {
                    new_trajectory.push(*i)
                }
                self.trajectory = new_trajectory;
            } else if let Direction::Down = self.direction {
                let mut new_trajectory = Vec::new();

                prevs.append(&mut nexts);
                for i in prevs {
                    new_trajectory.push(*i)
                }
                self.trajectory = new_trajectory;
            } else {
                if self.trajectory[0] > self.current_floor {
                    self.direction = Direction::Up
                } else {
                    self.direction = Direction::Down
                };
                return self.create_trajectory();
            }
        }

        pub fn go_to_basement(&mut self) -> () {
            println!("Going to basement");

            self.order = HashMap::new();
            self.requested = HashMap::new();
            self.trajectory = vec![self.basement];
        }

        pub fn run(&mut self) -> () {
            println!("Lift is running...");
            let mut instant = Instant::now();
            loop {
                self.get_request();
                if !self.trajectory.is_empty() {
                    self.create_trajectory();
                    self.go_next_floor();
                    self.empty_passenger();
                    self.get_passenger();
                }
                if self.trajectory.is_empty() {
                    if self.inactivity
                        && self.inactivity_time.as_secs() >= 4u64
                        && self.current_floor != self.basement
                    {
                        self.go_to_basement();
                    } else if self.inactivity {
                        let now = Instant::now();
                        self.inactivity_time += now - instant;
                        instant = now;
                    } else if !self.inactivity {
                        self.inactivity = true;
                        instant = Instant::now();
                        self.inactivity_time = Duration::new(0, 0);
                    }
                }
            }
        }
    }
}
