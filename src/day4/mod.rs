pub mod day4 {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    #[derive(Debug)]
    struct Square {
        value: u32,
        selected: bool,
    }

    impl Square {
        fn new(value: u32) -> Self {
            Self {
                value,
                selected: false,
            }
        }

        fn update_square(&mut self, draw: u32) {
            if self.value == draw {
                self.selected = true
            }
        }
    }

    struct Board {
        squares: Vec<Square>,
        won: bool,
    }

    impl Board {
        fn new(lines: Vec<String>) -> Self {
            let mut s = Self {
                squares: Vec::<Square>::new(),
                won: false,
            };

            for (_row, line) in lines.iter().enumerate() {
                for (_col, value) in line.split_whitespace().enumerate() {
                    s.squares.push(Square::new(value.parse().unwrap()));
                }
            }

            s
        }

        fn add_draw(&mut self, draw: u32) {
            for s in self.squares.iter_mut() {
                s.update_square(draw);
            }
        }

        fn bingo(&mut self) -> bool {
            if self.won {
                return false;
            }

            let mut row_result = true;
            let mut col_result = true;

            for row in 0..5 {
                for col in 0..5 {
                    row_result &= self.squares[row * 5 + col].selected == true;
                    col_result &= self.squares[col * 5 + row].selected == true;
                }

                if row_result == true || col_result == true {
                    self.won = true;
                    return true;
                } else {
                    row_result = true;
                    col_result = true;
                }
            }

            false
        }

        fn calculate_score(&self, draw: u32) -> u32 {
            let mut board_value = 0;

            for s in self
                .squares
                .iter()
                .filter(|s| s.selected == false)
                .collect::<Vec<&Square>>()
            {
                board_value += s.value;
            }

            board_value * draw
        }
    }

    fn read_boards() -> (Vec<u32>, Vec<Board>) {
        let file = File::open("./d4_input.txt").expect("File to be found");
        let reader = BufReader::new(file);
        let mut boards = Vec::<Board>::new();
        let mut first_line = Vec::<u32>::new();

        let mut lines = Vec::<String>::new();

        for line in reader.lines() {
            if first_line.len() == 0 {
                first_line = line
                    .unwrap()
                    .split(",")
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect();
            } else if lines.len() < 5 {
                let val = line.unwrap().trim().to_string();
                if val.len() > 0 {
                    lines.push(val);
                }
            } else {
                let b = Board::new(lines.clone());
                boards.push(b);
                lines.clear();
            }
        }

        let b = Board::new(lines.clone());
        boards.push(b);
        lines.clear();

        (first_line, boards)
    }

    pub fn puzzle1() {
        let (draws, mut boards) = read_boards();

        for d in draws {
            for b in boards.iter_mut() {
                b.add_draw(d);
                if b.bingo() == true {
                    println!("Score: {}", b.calculate_score(d));
                    return;
                }
            }
        }

        println!("No winners");
    }

    pub fn puzzle2() {
        let mut last_winning_board: u32 = 0;

        let (draws, mut boards) = read_boards();

        for d in draws {
            for b in boards.iter_mut() {
                b.add_draw(d);
                if b.bingo() == true {
                    last_winning_board = b.calculate_score(d);
                }
            }
        }

        println!("Score: {}", last_winning_board);
    }
}
