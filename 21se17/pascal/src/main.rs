// import io library to get inputs, and import the pascal module from the other file
use std::io::{self, Write};
mod pascal;

fn main() {
    // create a base pascal object. this has a single row (the vec provided), but more will be added later
    // change the vec to adjust how the triangle starts. remember to include the zeroes
    let mut x = pascal::Pascal::new(vec![0_u32, 1, 0]);

    // extend our pascal so it's a full triangle
    let height = int_input("pascal height : ");
    // if this input will cause an error due to its size, stop the program
    if height >= 36 { println!("this height will create a triangle which exceeds the confines of u32 (max 35)"); return; }

    x.extend(height - 1);

    // format and display the triangle
    println!();
    pascal::display(x);
}

// this gets an input and turns it into a u32
fn int_input(context: &str) -> u32 {
    print!("{}", context);

    let mut input = String::new();
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut input).expect("something went wrong when getting the input");
    let parsed_input: u32 = input.trim().parse::<u32>().expect("something went wrong when parsing");

    parsed_input
}
