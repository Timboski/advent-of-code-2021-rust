use std::cmp::Ordering;
use std::str::FromStr;
use std::fmt::Debug;

fn main() {
    println!("Sonar Sweep");
    let example_path = "/workspaces/advent-of-code-2021-rust/day1/src/puzzle1_input";
    let file_contents = read_all::<i32>(example_path);
    let res = sweep(&file_contents);
    println!("Answer: {}", res)
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

fn read_all<T: FromStr>(file_name: &str) -> Vec<T> where <T as FromStr>::Err: Debug {
    std::fs::read_to_string(file_name)
        .expect("file not found!")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect()
}

#[test]
fn test_example_sweep() {
    let v = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let res = sweep(&v);
    assert_eq!(res, 7);
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
