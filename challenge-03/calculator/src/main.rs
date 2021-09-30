mod number;
mod calculator;

fn main() {
    let run_fn = std::env::args().nth(1).expect("run function not found");
    let operation = run_fn.trim();
    let mut x = calculator::Calc::new((20, 0));

    x.cmd(operation);

    println!("{:?}, {}", x.x, operation)
}
