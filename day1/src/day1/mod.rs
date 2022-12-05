pub mod day1 {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    struct TripleDepth {
        first: Option<u32>,
        second: Option<u32>,
        third: Option<u32>,
    }

    impl TripleDepth {
        fn new() -> Self {
            Self {
                first: None,
                second: None,
                third: None,
            }
        }

        fn total(&self) -> u32 {
            let mut total = 0;

            if self.first.is_some() && self.second.is_some() && self.third.is_some() {
                total = self.first.unwrap() + self.second.unwrap() + self.third.unwrap();
            }

            total
        }

        fn push(&mut self, val: u32) {
            if self.first.is_none() {
                self.first = Some(val)
            } else if self.second.is_none() {
                self.second = self.first;
                self.first = Some(val)
            } else {
                self.third = self.second;
                self.second = self.first;
                self.first = Some(val)
            }
        }
    }

    fn read_depths() -> Vec<u32> {
        let file = File::open("./d1_input.txt").expect("File to be found");
        let reader = BufReader::new(file);
        let mut depths = Vec::<u32>::new();

        for line in reader.lines() {
            let d: u32 = line.unwrap().trim().parse().expect("A valid number");
            depths.push(d)
        }

        depths
    }

    pub fn puzzle1() {
        let depths = read_depths();

        let mut increase = 0;
        let mut prev = 0;

        for d in depths {
            if prev > 0 && d > prev {
                increase += 1;
            }

            prev = d;
        }

        println!("Number of increases: {}", increase);
    }

    pub fn puzzle2() {
        let depths = read_depths();

        let mut increase = 0;
        let mut prev = 0;
        let mut td = TripleDepth::new();

        for d in depths {
            td.push(d);

            if prev > 0 && td.total() > prev {
                increase += 1;
            }

            prev = td.total();
        }

        println!("Number of increases: {}", increase);
    }
}
