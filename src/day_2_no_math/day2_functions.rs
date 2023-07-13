use std::fs::read_to_string;

pub fn read_file(file_path: &str)-> String {
  let contents = read_to_string(file_path).expect("Should have been able to read the file");

  let mut buf = String::with_capacity(contents.len());
  for c in contents.chars() {
    buf.push(c);
  }
  return buf; 
}

pub fn solve_day2() {
  let file_path = "src/day_2_no_math/input.txt";
  let contents:Vec<String> = read_lines(file_path);

  let mut total_wrapping_paper:i32 = 0;
  let mut total_ribbon_length:i32 = 0;

  for line in contents{ //iterates through all presents (line) in contents (input-file) to add up the total wrapping paper and the total ribbon length
  
    //get_dimensions_from_line(line);
    let mut line_dimensions: [i32; 3] = get_dimensions_from_line(line); //line dimensions in an array with 3 indexes
  
    //println!("Line dimensions: {line_dimensions:?}");
    //println!("get_square_feet: {}", get_square_feet(line_dimensions));

    total_wrapping_paper += get_square_feet(line_dimensions);
    total_ribbon_length += get_ribbon_length(line_dimensions);
  }
  println!("Day2 Part1: Total wrapping paper: {total_wrapping_paper}");

  println!("Day2 Part2: Ribbon length: {total_ribbon_length}");
  }


fn get_dimensions_from_line(input:String)-> [i32; 3] { //saving each line of the input-file into an array, containing each number of the line (dimensions of one present)
let mut line_dimensions: [i32; 3] = [0, 0, 0];

  for (ind,element) in input.split("x").enumerate(){ //saves each number (dimension) of the line into an index
      //println!("Input-Element: {element}");

      line_dimensions[ind] = element.parse().unwrap(); 
    }

    return line_dimensions; //each line are the dimensions of one present
}


fn get_square_feet(input:[i32; 3]) -> i32 { //calculates the total size of a present (line)

  let mut lw:i32;
  let mut wh:i32;
  let mut hl:i32;

  lw = input[0]*input[1];
  wh = input[1]*input[2];
  hl = input[2]*input[0];

  let side_areas: [i32; 3] = [lw, wh, hl];

  let slack:i32 = *side_areas.iter().min().unwrap();
  // let slack:i32 = side_areas.min();

  return 2*lw + 2*wh + 2*hl + slack;
}


fn get_ribbon_length(mut input:[i32; 3]) -> i32 { //calculates the length of the ribbon of a present (line)

  input.sort();

  let ribbon_length = 2*input[0] + 2*input[1] + input[0]*input[1]*input[2];
  
  return ribbon_length;
}


fn read_lines(filename: &str) -> Vec<String> { //turns file into an array with each line as an index
  let mut result = Vec::new();

  for line in read_to_string(filename).unwrap().lines() {
      result.push(line.to_string())
  }

  result
}