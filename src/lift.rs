pub mod mechanic {
    use std::collections::HashMap;
    use std::sync::mpsc;
    use std::thread;
    use std::time;

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
        inactivity_time: time::Duration,
        basement: isize,
        floors: Vec<isize>,
        climbing_speed: time::Duration,
        descent_speed: time::Duration,
        current_floor: isize,
        order: HashMap<isize, Vec<Person>>,
        trajectory: Vec<isize>,
        direction: Direction,
        channel_receiver: mpsc::Receiver<(isize, String, isize)>,
    }

    impl Lift {
        pub fn new(
            floors: Vec<isize>,
            speed: (time::Duration, time::Duration),
            rx: mpsc::Receiver<(isize, String, isize)>,
        ) -> Lift {
            let basement = floors[0];
            Lift {
                inactivity: true,
                inactivity_time: time::Duration::new(0, 0),
                basement,
                floors,
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
            for i in &self.order[&self.current_floor] {
                println!("Passenger {} you can get down. Have a nice day!", i.name);
            }
            let to = self.order.entry(self.current_floor);
            to.or_default();
        }

        fn get_request(&mut self) -> () {
            loop {
                let person = self.channel_receiver.try_recv();
                match person {
                    Ok((floor, name, dest)) => {
                        let p = Person { name, floor: dest };
                        self.order.entry(floor).or_default().push(p);
                        if !self.trajectory.contains(&floor) {
                            self.trajectory.push(floor);
                        }
                    }
                    Err(_) => return,
                }
            }
        }

        fn create_trajectory(&mut self) -> () {
            if let Direction::Up = self.direction {
                let mut prevs = self
                    .trajectory
                    .drain_filter(|e| e < &mut self.current_floor)
                    .collect::<Vec<_>>();
                self.trajectory.sort();
                prevs.sort_by(|a, b| b.cmp(a));
                self.trajectory.append(&mut prevs);
            } else if let Direction::Down = self.direction {
                let mut nexts = self
                    .trajectory
                    .drain_filter(|e| e > &mut self.current_floor)
                    .collect::<Vec<_>>();
                self.trajectory.sort_by(|a, b| b.cmp(a));
                nexts.sort();
                self.trajectory.append(&mut nexts);
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

            // self.requested = HashMap::new();
            self.trajectory = vec![self.basement];
        }

        pub fn run(&mut self) -> () {
            loop {
                self.get_request();
                self.create_trajectory();
                self.go_next_floor();
                self.empty_passenger();
            }
        }
    }
}
