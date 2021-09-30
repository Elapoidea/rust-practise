use crate::number;

pub struct Calc {
    pub x: (i64, i64)
}

impl Calc {
    pub fn new(x: (i64, i64)) -> Self {
        Calc { x }
    }

    pub fn cmd(&mut self, command: &str) {
        let cmd_result = match command {
            "lcm" => Ok(number::lcm(self.x)),
            "gcf" => Ok(number::gcf(self.x)),
            "factorial" | "!" => Ok(number::factorial(self.x)),
            _ => Err("not a valid command"),
        };

        self.x = match cmd_result {
            Ok(n) => (n, 0),
            Err(_) => self.x,
        };
    }
}
