mod automata;
mod space;

/*
0000000
0000000
0000000
0000000
1110000
0010000
0100000
*/

fn main() {
    let mut x = space::Space::new(7, 7, "0000000000000000000000000000111000000100000100000");

    for _ in 0..15 {
        x.display();
        x.advance();
        println!();
    }

}
