use std::process::abort;

fn main() {
    println!("Binary Diagnostic");
}

fn decode_diagnostic(diagnostic: &[&str]) -> u32 {
    let mut total: u32 = 0;
    let num_bits = diagnostic[0].len();
    for i in 0..num_bits {
        match get_bit(i, diagnostic) {
            true => {
                let bit_value = 1 << (num_bits - i - 1);
                println!("Bit {} = {}", i, bit_value);
                total += bit_value
            }
            false => {}
        };
    }

    let mask: u32 = ((1 << num_bits) - 1).try_into().unwrap();
    let inv = !total & mask;
    total * inv
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
fn test_example_diagnostic_from_file() {
    // Arrange
    let path = "/workspaces/advent-of-code-2021-rust/day3/src/test_example";
    let file = std::fs::read_to_string(path).expect("file not found!");
    let lines: Vec<&str> = file.lines().collect();

    // Act
    let res = decode_diagnostic(&lines);

    // Assert
    assert_eq!(res, 198);
}
