mod Mechanic {
    use std::thread;
    use std::time;

    #[derive(Debug)]
    pub struct Person {
        name: String,
        floor: Option<isize>,
    }

    #[derive(Debug)]
    pub struct Lift<'a> {
        inactivity: bool,
        // passengers: Vec<Person>,
        requested: Vec<(bool, Person)>,
        inactivity_time: time::Duration,
        basement: isize,
        floors: &'a [isize],
        climbing_speed: time::Duration,
        descent_speed: time::Duration,
        current_floor: isize,
        trajectory: Vec<(isize, Option<Vec<Person>>)>,
    }

    impl<'a> Lift<'a> {
        pub fn new(floors: &'a [isize], speed: (time::Duration, time::Duration)) -> Lift<'a> {
            Lift {
                inactivity: true,
                // passengers: Vec::new(),
                requested: Vec::new(),
                inactivity_time: time::Duration::new(0, 0),
                basement: floors[0],
                floors,
                climbing_speed: speed.0,
                descent_speed: speed.1,
                current_floor: floors[0],
                trajectory: Vec::new(),
            }
        }

        pub fn get_current_floor(&self) -> isize {
            self.current_floor
        }

        fn go_next_floor(&mut self) -> () {
            if self.trajectory.is_empty() {
                return;
            }
            let target = &self.trajectory[0];
            thread::sleep(if target.0 > self.current_floor {
                self.climbing_speed
            } else {
                self.descent_speed
            });
            self.current_floor = target.0;
        }

        fn empty_passenger(&mut self) -> () {
            let passengers = self
                .trajectory
                .iter()
                .filter(|&e| e.0 == self.current_floor)
                .next()
                .unwrap();
            println!("Floor {}", self.current_floor);
            if let Some(x) = &passengers.1 {
                for i in x {
                    println!("Passenger {} you can get down. Have a nice day!", i.name);
                }
            }
        }

        // fn get_passengers(&mut self) -> () {

        // }

        pub fn go_to_basement(&mut self) -> () {
            println!("Going to basement");
            self.trajectory = vec![(self.basement, None)];
        }
    }
}
