use std::process::abort;

fn main() {
    println!("Dive!");

    let path = "/workspaces/advent-of-code-2021-rust/day2/src/puzzle_input";
    let file = std::fs::read_to_string(path).expect("file not found!");
    let lines: Vec<&str> = file.lines().collect();

    let res = plot_course(&lines);
    println!("Example result: {}", res)
}

fn plot_course(course: &[&str]) -> i32 {
    let mut depth = 0;
    let mut horizontal = 0;
    for it in course {
        let step: Vec<&str> = it.split_ascii_whitespace().collect();
        let distance: i32 = step[1].parse().expect("Invalid Distance");
        match step[0] {
            "forward" => horizontal += distance,
            "down" => depth += distance,
            "up" => depth -= distance,
            _ => abort(),
        }
    }
    depth * horizontal
}

#[test]
fn test_example_route() {
    // Arrange
    let course = vec![
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ];

    // Act
    let res = plot_course(&course);

    // Assert
    assert_eq!(res, 150);
}

#[test]
fn test_example_route_from_file() {
    // Arrange
    let path = "/workspaces/advent-of-code-2021-rust/day2/src/test_example";
    let file = std::fs::read_to_string(path).expect("file not found!");
    let lines: Vec<&str> = file.lines().collect();

    // Act
    let res = plot_course(&lines);

    // Assert
    assert_eq!(res, 150);
}
