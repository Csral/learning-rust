use std::io;

fn main() {

    println!("Guessing Game!!!");
    println!("Enter your guess> ");

    let mut ans = String::new();

    io::stdin().read_line(&mut ans).expect("Failed to read line.");

    println!("Your number> {}", ans);

}