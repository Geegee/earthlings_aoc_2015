use std::collections::HashMap;
use std::fs::read_to_string;

pub fn read_file(file_path: &str) -> String {
    let contents = read_to_string(file_path).expect("Should have been able to read the file");

    let mut buf = String::with_capacity(contents.len());
    for c in contents.chars() {
        buf.push(c);
    }
    return buf;
}

pub fn solve_day3() {
    let file_path = "src/day_3_spherical_houses/input.txt";
    let contents: String = read_file(file_path);

    let mut coordinate_hash: HashMap<(i32, i32), i32> = HashMap::new();
    let mut position: (i32, i32) = (0, 0);
    let mut robot_position: (i32, i32) = (0, 0);
    let mut switch: bool = true; //true = santa, false = robot

    coordinate_hash.insert((0, 0), 1);

    for move_command in contents.chars() {
        let mut temp_position;

        if switch {
            position = move_santa(position, move_command);

            temp_position = position;
        } else {
            robot_position = move_santa(robot_position, move_command);
            temp_position = robot_position;
        }
        switch = !switch;

        if coordinate_hash.contains_key(&temp_position) {
            *coordinate_hash.get_mut(&temp_position).unwrap() += 1;
        } else {
            coordinate_hash.insert(temp_position, 1);
        }
    }

    println!(
        "Number of different houses (coordinates): {:?}",
        coordinate_hash.len()
    );

    // println!("new position: {:?}", move_santa((0, 0), "^"));
    // println!("Input: {contents}");
}

fn move_santa(position: (i32, i32), direction: char) -> (i32, i32) {
    match direction {
        '^' => return (position.0, position.1 + 1),
        '>' => return (position.0 + 1, position.1),
        'v' => return (position.0, position.1 - 1),
        '<' => return (position.0 - 1, position.1),
        _ => println!("no valid move"),
    }

    return (0, 0);
}
