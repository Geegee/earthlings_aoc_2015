pub mod day1_functions {
use std::fs;

pub fn read_file(file_path: &str)-> String {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let mut buf = String::with_capacity(contents.len());
    for c in contents.chars() {
        buf.push(c);
    }

    return buf;
}

pub fn solve_day1() {
    // --snip--
    let file_path = "/home/geegee/org/roam/Programming/Rust/earthlings_aoc_2015/src/day_1_not_quite_lisp/input.txt";
    let contents:String = read_file(file_path);
    let mut santa_floor:i32 = 0;
    let mut pos:i32 = 0;
    let mut found:bool = false;
    for i in contents.chars() {
        pos += 1;
        if i == '(' {
            santa_floor += 1;
        } else if i == ')' {
            santa_floor -= 1;}
        if santa_floor == -1 && !found {
            print!("First position Santa is in floor -1: {pos} \n");
            found = true;
        }

        //        print!("{}", i);
    }
    print!("Final floor Santa ends up in: {santa_floor}") 
}}
