use std::process::abort;

fn main() {
    println!("Dive!");

    let path = "/workspaces/advent-of-code-2021-rust/day2/src/puzzle_input";
    let file = std::fs::read_to_string(path).expect("file not found!");
    let lines: Vec<&str> = file.lines().collect();

    let res1 = plot_course(&lines);
    println!("Puzzle 1 result: {}", res1);

    let res2 = plot_course_scheme2(&lines);
    println!("Puzzle 2 result: {}", res2);
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

fn plot_course_scheme2(course: &[&str]) -> i32 {
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;
    for it in course {
        let step: Vec<&str> = it.split_ascii_whitespace().collect();
        let distance: i32 = step[1].parse().expect("Invalid Distance");
        match step[0] {
            "forward" => {
                horizontal += distance;
                depth += aim * distance;
            }
            "down" => aim += distance,
            "up" => aim -= distance,
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

#[test]
fn test_example_route_using_new_scheme() {
    // Arrange
    let path = "/workspaces/advent-of-code-2021-rust/day2/src/test_example";
    let file = std::fs::read_to_string(path).expect("file not found!");
    let lines: Vec<&str> = file.lines().collect();

    // Act
    let res = plot_course_scheme2(&lines);

    // Assert
    assert_eq!(res, 900);
}

#[test]
fn puzzle1_regression() {
    // Arrange
    let path = "/workspaces/advent-of-code-2021-rust/day2/src/puzzle_input";
    let file = std::fs::read_to_string(path).expect("file not found!");
    let lines: Vec<&str> = file.lines().collect();

    // Act
    let res = plot_course(&lines);

    // Assert
    assert_eq!(res, 1813801);
}
