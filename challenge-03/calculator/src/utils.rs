use std::io::{self, Write};

pub fn get_input(context: Option<&str>) -> String {
    let mut input = String::new();

    match context {
        Some(n) => {
            print!("{}", n);

            let _ = io::stdout().flush();
        },
        None => print!(""),
    }

    io::stdin().read_line(&mut input).expect("something went wrong when getting the input");

    input.trim().to_string()
}

pub fn get_trinary_input(context: Option<&str>) -> u8 {
    let number: u8;

    loop {
         number = match get_input(context).parse() {
            Ok(n) => { if n <= 2 && n >= 0 { n } else { continue } },
            Err(_) => continue,
        };

        break
    };

    number
}

pub fn get_int_input(context: Option<&str>) -> i64 {
    let number: i64;

    loop {
         number = match get_input(context).parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        break
    };

    number
}
