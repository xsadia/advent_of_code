use std::fs;
use std::io::{BufReader, BufRead};

fn main() {
  let filename = "input.txt";
  let file = fs::File::open(filename).unwrap();
  let reader = BufReader::new(file); 
  let mut prev: i32 = 0;
  let mut acc = 0;

  for (index, line) in reader.lines().enumerate() {
    let curr_line = String::from(line.unwrap());
    let parsed_line = curr_line.parse::<i32>().unwrap();

    match index {
      0 => prev = parsed_line,
      _ => if parsed_line > prev {
        println!("increased");
        acc += 1;
        prev = parsed_line;
      } else if parsed_line < prev {
        println!("decreased");
        prev = parsed_line;
      }
    }
  }

  println!("{}", acc);
}