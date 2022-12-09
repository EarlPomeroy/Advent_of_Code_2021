pub mod day5 {
    use std::{
        cmp,
        fs::File,
        io::{BufRead, BufReader},
    };
    #[derive(Clone, Debug)]
    struct Line {
        line: Vec<Point>,
    }

    impl Line {
        fn new(x1: u32, y1: u32, x2: u32, y2: u32) -> Self {
            let mut points = Vec::<Point>::new();

            if x1 == x2 {
                for y in y1..=y2 {
                    points.push(Point::new(x1, y));
                }
            } else {
                for x in x1..=x2 {
                    points.push(Point::new(x, y1));
                }
            }

            Self { line: points }
        }

        fn calculate_overlap(&self, other: Line) -> u32 {
            let mut count = 0;

            for p in other.line.iter() {
                if self.line.contains(p) {
                    count += 1;
                }
            }

            count
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Point {
        x: u32,
        y: u32,
    }

    impl Point {
        fn new(x: u32, y: u32) -> Self {
            Self { x, y }
        }
    }

    fn is_valid(line: &Vec<u32>) -> bool {
        if line.len() == 4 && (line[0] == line[2] || line[1] == line[3]) {
            return true;
        }

        false
    }

    pub fn puzzle1() {
        let mut lines = Vec::<Line>::new();

        let file = File::open("./d5_input.txt").expect("File to be found");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let points = line
                .unwrap()
                .replace(" -> ", ",")
                .split(",")
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            if is_valid(&points) {
                lines.push(Line::new(
                    cmp::min(points[0], points[2]),
                    cmp::min(points[1], points[3]),
                    cmp::max(points[0], points[2]),
                    cmp::max(points[1], points[3]),
                ));
            }
        }

        let mut overlap = 0;

        println!("{}", lines.len());

        for (i, line) in lines.iter().enumerate() {
            println!("{}", i);
            for l in i + 1..lines.len() {
                overlap += line.calculate_overlap(lines[l].clone());
            }
        }

        println!("{}", overlap);
    }

    pub fn puzzle2() {}
}
