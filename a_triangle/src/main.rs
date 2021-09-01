// This program prints 10 stars '*' in one line.
// Change the program to:
// - read the stars_width number from user using the `get_number_from_user` function
// - print a triangle made of '*':
//   for e.g. stars_width=3 it would give the output:
//   *
//   **
//   ***

use std::io;

fn get_number_from_user() -> u32 {
    println!("Choose a number for stars width");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read from stdin!");
    guess.trim().parse::<u32>().expect("Please type a number!")
}

fn main() {
    let stars_width = 10;

    for _i in 0..stars_width {
        print!("*");
    }
    println!();
}
