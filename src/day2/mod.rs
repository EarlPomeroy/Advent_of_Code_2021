pub mod day2 {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    enum Direction {
        Forward,
        Down,
        Up,
        Unknown,
    }

    struct Movement {
        distance: u32,
        direction: Direction,
    }

    impl Movement {
        fn new(line: &str) -> Self {
            let mut parts = line.split_whitespace();

            let dir = match parts.next().unwrap() {
                "forward" => Direction::Forward,
                "down" => Direction::Down,
                "up" => Direction::Up,
                _ => Direction::Unknown,
            };

            let dist: u32 = parts.next().unwrap().parse().expect("A valid number");

            Self {
                distance: dist,
                direction: dir,
            }
        }
    }

    fn read_movements() -> Vec<Movement> {
        let file = File::open("./d2_input.txt").expect("File to be found");
        let reader = BufReader::new(file);
        let mut movements = Vec::<Movement>::new();

        for line in reader.lines() {
            let m = Movement::new(line.unwrap().trim());
            movements.push(m);
        }

        movements
    }

    pub fn puzzle1() {
        let movements = read_movements();
        let mut x = 0;
        let mut y = 0;

        for m in movements {
            match m.direction {
                Direction::Forward => x += m.distance,
                Direction::Up => y -= m.distance,
                Direction::Down => y += m.distance,
                _ => {}
            }
        }

        println!("Total Area: {}", x * y);
    }

    pub fn puzzle2() {
        let movements = read_movements();
        let mut x = 0;
        let mut y = 0;
        let mut aim: u32 = 0;

        for m in movements {
            match m.direction {
                Direction::Forward => {
                    x += m.distance;
                    if aim > 0 {
                        y += aim * m.distance;
                    }
                }
                Direction::Up => aim -= m.distance,
                Direction::Down => aim += m.distance,
                _ => {}
            }
        }

        println!("Total Area: {}", x * y);
    }
}
