// DAY02: https://adventofcode.com/2015/day/2

use std::fs::File;
use std::io::{self, BufRead};

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY02 -------");
    let input = read_input("inputs/input_day02");
    day02_part1(&input);
    day02_part2(&input);
}

fn read_input(path: &str) -> Vec<Box> {
    let mut input: Vec<Box> = Vec::new();
    if let Ok(file) = File::open(path) {
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            let new_box = Box::read_box(&line.unwrap());
            input.push(new_box);
        }
    }
    input
}

fn day02_part1(input: &Vec<Box>) {
    // Exemple tests
    let test_box = Box::read_box("2x3x4");
    assert_eq!(test_box.wrapping_paper(), 58);
    let test_box = Box::read_box("1x1x10");
    assert_eq!(test_box.wrapping_paper(), 43);

    // Solve puzzle
    let mut s: u32 = 0;
    for new_box in input {
        s += new_box.wrapping_paper();
    }
    // println!("Result part 1: {s}");
    assert_eq!(s, 1606483);
    println!("> DAY02 - part 1: OK!");
}

fn day02_part2(input: &Vec<Box>) {
    // Exemple tests
    let test_box = Box::read_box("2x3x4");
    assert_eq!(test_box.ribbon(), 34);
    let test_box = Box::read_box("1x1x10");
    assert_eq!(test_box.ribbon(), 14);

    // Solve puzzle
    let mut s: u32 = 0;
    for new_box in input {
        s += new_box.ribbon();
    }
    // println!("Result part 2: {s}");
    assert_eq!(s, 3842356);
    println!("> DAY02 - part 2: OK!");
}

struct Box {
    length: u32,
    width: u32,
    height: u32,
}

impl Box {
    fn read_box(dimensions: &str) -> Box {
        let dims: Vec<&str> = dimensions.split('x').collect();
        Box {
            length: dims[0].parse().unwrap(),
            width: dims[1].parse().unwrap(),
            height: dims[2].parse().unwrap(),
        }
    }

    fn get_side_areas(&self) -> Vec<u32> {
        vec![
            self.length * self.width,
            self.length * self.height,
            self.width * self.height,
        ]
    }

    fn smallest_side_area(&self) -> u32 {
        self.get_side_areas().iter().copied().min().unwrap()
    }

    fn wrapping_paper(&self) -> u32 {
        let sum: u32 = self.get_side_areas().iter().sum();
        2 * sum + self.smallest_side_area()
    }

    fn cubic(&self) -> u32 {
        self.length * self.width * self.height
    }

    fn get_side_perimeters(&self) -> Vec<u32> {
        vec![
            self.length * 2 + self.width * 2,
            self.length * 2 + self.height * 2,
            self.width * 2 + self.height * 2,
        ]
    }

    fn smallest_side_perimeter(&self) -> u32 {
        self.get_side_perimeters().iter().copied().min().unwrap()
    }

    fn ribbon(&self) -> u32 {
        self.smallest_side_perimeter() + self.cubic()
    }
}
