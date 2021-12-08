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
    let oxygen_generator_rating = find_match(diagnostic, true);
    let co2_scrubber_rating = find_match(diagnostic, false);
    oxygen_generator_rating * co2_scrubber_rating
}

fn find_match(diagnostic: &[&str], criteria: bool) -> u32 {
    let num_bits = diagnostic[0].len();
    let mut filtered: Vec<&str> = diagnostic.to_vec();
    for i in 0..num_bits {
        let filter = get_bit_match(i, criteria, &filtered);
        filtered = filtered
            .iter()
            .filter(|a| a.chars().nth(i).unwrap() == filter)
            .cloned()
            .collect();
        println!("Filter bit {} on {} List: {:?}", i, filter, filtered);

        if filtered.len() == 1 {
            println!("Found {:?}", filtered);
            return u32::from_str_radix(filtered[0], 2).unwrap();
        };
    }
    abort();
}

fn get_bit(i: usize, diagnostic: &[&str]) -> bool {
    bit_weighting(i, diagnostic) > 0
}

fn get_bit_match(i: usize, criteria: bool, diagnostic: &[&str]) -> char {
    let weight = bit_weighting(i, diagnostic);
    let one = if criteria { weight >= 0 } else { weight < 0 };
    if one {
        '1'
    } else {
        '0'
    }
}

fn bit_weighting(i: usize, diagnostic: &[&str]) -> i32 {
    diagnostic
        .iter()
        .map(|a| match a.chars().nth(i).unwrap() {
            '0' => -1,
            '1' => 1,
            _ => abort(),
        })
        .sum()
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
