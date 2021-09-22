use std::io::{self, Write};
mod pascal;

fn main() {
    let mut x = pascal::Pascal::new(vec![0_u32, 1, 0]);

    x.extend(int_input("pascal height : ") - 1);

    println!();

    pascal::display(x);
}

fn int_input(context: &str) -> u32 {
    print!("{}", context);

    let mut input = String::new();
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut input).expect("something went wrong when getting the input");
    let parsed_input: u32 = input.trim().parse::<u32>().expect("something went wrong when parsing");

    parsed_input
}
