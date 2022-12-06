pub mod day3 {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    fn get_readings() -> Vec<String> {
        let file = File::open("./d3_input.txt").expect("File to be found");
        let reader = BufReader::new(file);
        let mut lines = Vec::<String>::new();

        for line in reader.lines() {
            lines.push(line.unwrap().trim().to_string());
        }

        lines
    }

    #[derive(PartialEq)]
    enum Mode {
        Most,
        Least,
    }

    fn calculate_power(readings: &Vec<String>, mode: Mode) -> u32 {
        let mut power = 0;
        let len = readings[0].len() as u32;
        let mut idx = 0;

        while idx < len {
            let mut zero_bit = 0;
            let mut one_bit = 0;

            for line in readings {
                if line.as_bytes()[idx as usize] == b'0' {
                    zero_bit += 1;
                } else {
                    one_bit += 1;
                }
            }

            let mut result = 0;
            if mode == Mode::Most {
                result = if zero_bit > one_bit { 0 } else { 1 };
            } else {
                result = if zero_bit < one_bit { 0 } else { 1 };
            }

            if result == 1 {
                power = power + u32::pow(2, len - idx - 1);
            }

            idx += 1;
        }

        power
    }

    fn calculate_gas(readings: &Vec<String>, mode: Mode) -> u32 {
        let mut filter = "".to_string();
        let len = readings[0].len() as u32;
        let mut idx = 0;

        let mut current_set = readings.clone();

        while idx < len && current_set.len() > 1 {
            let mut zero_bit = 0;
            let mut one_bit = 0;

            for line in &current_set {
                if line.as_bytes()[idx as usize] == b'0' {
                    zero_bit += 1;
                } else {
                    one_bit += 1;
                }
            }

            if mode == Mode::Most {
                filter += if zero_bit > one_bit { "0" } else { "1" };
            } else {
                filter += if zero_bit <= one_bit { "0" } else { "1" };
            }

            current_set.retain(|r| r.starts_with(&filter));

            idx += 1;
        }

        u32::from_str_radix(current_set[0].as_str(), 2).unwrap()
    }

    pub fn puzzle1() {
        let readings = get_readings();

        let gamma_rate = calculate_power(&readings, Mode::Most);
        let epsilon_rate = calculate_power(&readings, Mode::Least);

        println!("power consumption: {}", gamma_rate * epsilon_rate);
    }

    pub fn puzzle2() {
        let readings = get_readings();

        let oxygen_generator = calculate_gas(&readings, Mode::Most);
        let c02_scrubber = calculate_gas(&readings, Mode::Least);

        println!("power consumption: {}", oxygen_generator * c02_scrubber);
    }
}
