use crate::number;
use crate::utils;

const help_message: &str = "add           : adds the two numbers together
sub           : subtracts the left number by the right number
mult          : multiplies the left number by the right number
div           : divides the left number the right number. result is the left number, if the right number is 0
factorial | ! : gets the factorial of the left number until the right number
lcm           : gets the least common multiple of the two numbers
gcf           : gets the greatest common factor of the two numbers
let           : assigns a value to a slot";

pub struct Calc {
    pub x: (i64, i64)
}

impl Calc {
    pub fn new(x: (i64, i64)) -> Self {
        Calc { x }
    }

    pub fn cmd(&mut self, command: &str) {
        let cmd_result = match command {
            "add" | "+" => number::add(self.x),
            "sub" | "-" => number::sub(self.x),
            "mult" | "*" => number::mult(self.x),
            "div" | "/" => number::div(self.x),
            "factorial" | "!" => number::factorial(self.x),
            "lcm" => number::lcm(self.x),
            "gcf" => number::gcf(self.x),
            "let" => utils::get_int_input(Some(" value > ")),
            "help" => { println!("{}", help_message); return },
            _ => { println!("not a valid command"); return },
        };

        self.x = match utils::get_trinary_input(Some(" slot  > ")) {
            0 => (cmd_result, self.x.1),
            1 => (self.x.0, cmd_result),
            2 => { println!(" ^ {:?}", cmd_result); self.x }
            _ => unreachable!(),
        };
    }
}
