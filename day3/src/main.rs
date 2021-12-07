fn main() {
    println!("Binary Diagnostic");
}

fn decode_diagnostic(diagnostic: &[&str]) -> i32 {
    0
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