fn main() {
    println!("Giant Squid");

    let path = "/workspaces/advent-of-code-2021-rust/day4/src/test_example";
    let file = std::fs::read_to_string(path).expect("file not found!");
    let mut lines = file.lines();

    let calls: Vec<i32> = decode_line(lines.next().unwrap(), ',');
    println!("Calls: {:?}", calls);

    while let Some(blank) = lines.next() {
        assert!(blank.is_empty());
        println!();
        println!("Board:");
        for _ in 0..5 {
            let board_row = decode_line(lines.next().unwrap(), ' ');
            println!("{:?}", board_row);
        }
    }
}

fn decode_line(line: &str, delimeter: char) -> Vec<i32> {
    line.split(delimeter)
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}
