use std::process::abort;

fn main() {
    println!("Binary Diagnostic");

    let path = "/workspaces/advent-of-code-2021-rust/day3/src/puzzle_input";
    let file = std::fs::read_to_string(path).expect("file not found!");
    let lines: Vec<&str> = file.lines().collect();

    let res1 = decode_power_consumption(&lines);
    println!("Puzzle 1 result: {}", res1);

    let res2 = decode_life_support_rating(&lines);
    println!("Puzzle 2 result: {}", res2);
}

fn decode_power_consumption(diagnostic: &[&str]) -> u32 {
    let mut total: u32 = 0;
    let num_bits = diagnostic[0].len();
    for i in 0..num_bits {
        if get_bit(i, diagnostic) {
            total += 1 << (num_bits - i - 1)
        };
    }

    let mask: u32 = ((1 << num_bits) - 1).try_into().unwrap();
    let inv = !total & mask;
    total * inv
}

fn decode_life_support_rating(diagnostic: &[&str]) -> u32 {
    0
}

fn get_bit(i: usize, diagnostic: &[&str]) -> bool {
    let sum: i32 = diagnostic
        .iter()
        .map(|a| match a.chars().nth(i).unwrap() {
            '0' => -1,
            '1' => 1,
            _ => abort(),
        })
        .sum();
    sum > 0
}

#[test]
fn test_get_bit() {
    // Arrange
    let diagnostic = ["011", "001", "100"];

    // Act
    let res0 = get_bit(0, &diagnostic);
    let res1 = get_bit(1, &diagnostic);
    let res2 = get_bit(2, &diagnostic);

    // Assert
    assert!(!res0);
    assert!(!res1);
    assert!(res2);
}

#[test]
fn test_power_consumption_from_example_diagnostic_file() {
    // Arrange
    let path = "/workspaces/advent-of-code-2021-rust/day3/src/test_example";
    let file = std::fs::read_to_string(path).expect("file not found!");
    let lines: Vec<&str> = file.lines().collect();

    // Act
    let res = decode_power_consumption(&lines);

    // Assert
    assert_eq!(res, 198);
}

#[test]
fn test_life_support_rating_from_example_diagnostic_file() {
    // Arrange
    let path = "/workspaces/advent-of-code-2021-rust/day3/src/test_example";
    let file = std::fs::read_to_string(path).expect("file not found!");
    let lines: Vec<&str> = file.lines().collect();

    // Act
    let res = decode_life_support_rating(&lines);

    // Assert
    assert_eq!(res, 230);
}

#[test]
fn puzzle1_regression() {
    // Arrange
    let path = "/workspaces/advent-of-code-2021-rust/day3/src/puzzle_input";
    let file = std::fs::read_to_string(path).expect("file not found!");
    let lines: Vec<&str> = file.lines().collect();

    // Act
    let res = decode_power_consumption(&lines);

    // Assert
    assert_eq!(res, 1071734);
}
