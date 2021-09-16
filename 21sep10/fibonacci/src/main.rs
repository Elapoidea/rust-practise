use std::io;

fn main() {
    // get user input as String, and parse it to u64
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("something went wrong when getting the input");
    let n: u64 = input.trim().parse::<u64>().expect("something went wrong converting the input");

    let fib = fibonacci(n);

    println!();

    for x in &fib {
        println!("{}", x);
    }
}

fn fibonacci(upto: u64) -> Vec<u64> {
    let mut series = vec![1, 1];

    // offset the range one forward so there is always a previous value in the series
    // and offset it one backwards to make sure that it will calculate the right amount of values
    for i in 1..upto-1 {
        series.push({
            let a = series[i as usize];
            let b = series[(i - 1) as usize];

            a + b
        });
    }

    series
}
