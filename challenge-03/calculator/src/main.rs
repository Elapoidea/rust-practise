mod utils;
mod number;
mod calculator;

fn main() {
    let mut x = calculator::Calc::new((0, 0));

    loop {
        let operation = utils::get_input(Some(" cmd   > "));

        x.cmd(&operation);

        println!(" ^ {:?}\n", x.x)
    }
}
