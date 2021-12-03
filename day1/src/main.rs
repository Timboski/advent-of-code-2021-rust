use std::cmp::Ordering;
use std::str::FromStr;
use std::fmt::Debug;

fn main() {
    println!("Sonar Sweep");
    let example_path = "/workspaces/advent-of-code-2021-rust/day1/src/puzzle_input";
    let file_contents = read_all::<i32>(example_path);

    println!("Puzzle 1");
    let res1 = sweep(&file_contents);

    
    println!("Puzzle 1");
    let filtered = sliding_window(&file_contents);
    let res2 = sweep(&filtered);
    
    println!("Puzzle 1 Answer: {}", res1);
    println!("Puzzle 2 Answer: {}", res2)
}

pub fn sweep(v: &[i32]) -> i32 {
    let mut count: i32 = 0;
    let mut current_depth: i32 = v[0];

    for depth in v {
        print!("{}", depth);

        match depth.cmp(&current_depth) {
            Ordering::Less => println!(" => Smaller!"),
            Ordering::Greater => {
                println!(" => Bigger!");
                count += 1;
            }
            Ordering::Equal => println!(" => No Change!"),
        }
        
        current_depth = *depth;
    }
    count
}

fn sliding_window(v: &[i32]) -> Vec<i32> {
    let size = v.len();
    v[0..(size -2)].iter()
        .zip(v[1..(size - 1)].iter())
        .zip(v[2..size].iter())
        .map(|((a,b),c)| a + b + c)
        .collect()
}

fn read_all<T: FromStr>(file_name: &str) -> Vec<T> where <T as FromStr>::Err: Debug {
    std::fs::read_to_string(file_name)
        .expect("file not found!")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect()
}

#[test]
fn test_example_sweep() {
    // Arrange
    let v = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    // Act
    let res = sweep(&v);

    // Assert
    assert_eq!(res, 7);
}

#[test]
fn test_sliding_window_example() {
    // Arrange
    let v = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let expected = vec![607, 618, 618, 617, 647, 716, 769, 792];

    // Act
    let filtered = sliding_window(&v);

    // Assert
    assert_eq!(filtered.len(), expected.len(), "Arrays don't have the same length");
    assert!(filtered.iter().zip(expected.iter()).all(|(a,b)| a == b), "Arrays are not equal");
}

#[test]
fn test_example_file() {
    // Arrange
    let expected = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let example_path = "/workspaces/advent-of-code-2021-rust/day1/src/test_example";

    // Act
    let file_contents = read_all::<i32>(example_path);

    // Assert
    assert_eq!(file_contents.len(), expected.len(), "Arrays don't have the same length");
    assert!(file_contents.iter().zip(expected.iter()).all(|(a,b)| a == b), "Arrays are not equal");
}

#[test]
fn puzzle1_regression() {
    // Arrange
    let example_path = "/workspaces/advent-of-code-2021-rust/day1/src/puzzle_input";
    let file_contents = read_all::<i32>(example_path);

    // Act
    let res = sweep(&file_contents);

    // Assert
    assert_eq!(res, 1532);
}

#[test]
fn puzzle2_regression() {
    // Arrange
    let example_path = "/workspaces/advent-of-code-2021-rust/day1/src/puzzle_input";
    let file_contents = read_all::<i32>(example_path);

    // Act
    let filtered = sliding_window(&file_contents);
    let res = sweep(&filtered);

    // Assert
    assert_eq!(res, 1571);
}
