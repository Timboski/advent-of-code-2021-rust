use std::str::Lines;

fn main() {
    println!("Giant Squid");

    let path = "/workspaces/advent-of-code-2021-rust/day4/src/test_example";
    let file = std::fs::read_to_string(path).expect("file not found!");
    let mut lines = file.lines();

    let calls: Vec<i32> = lines.next().unwrap().decode_line(',');
    println!("Calls: {:?}", calls);

    while let Some(blank) = lines.next() {
        assert!(blank.is_empty());
        println!();
        println!("Board:");
        let board = BingoBoard::new(&mut lines);
        board.print();
    }
}

pub trait DecodeLine {
    fn decode_line(&self, delimeter: char) -> Vec<i32>;
}

impl DecodeLine for &str {
    fn decode_line(&self, delimeter: char) -> Vec<i32> {
        self.split(delimeter)
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect()
    }
}

struct BingoBoard {
    rows: Vec<Vec<i32>>,
}

impl BingoBoard {
    fn new(lines: &mut Lines) -> Self {
        let rows = (0..5)
            .map(|_| lines.next().unwrap().decode_line(' '))
            .collect();
        Self { rows }
    }

    fn print(&self) {
        for row in &self.rows {
            println!("{:?}", row);
        }
    }
}
