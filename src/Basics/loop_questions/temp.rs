use std::io;

fn main() {

    let mut input: String = String::new();
    println!("Enter temp in C> ");
    io::stdin().read_line(&mut input).expect("Unable to read line!");
    let input: f32 = input.trim().parse().expect("Not a number!");

    // f = 32 + 9/5 * C

    let deg_F = convert_to_F(&input);

    println!("{input} Celsius = {deg_F} fahrenheit");

}

fn convert_to_F(deg_C: &f32) -> f32 {

    /* Borrow temp */
    32.0 + (1.8 * deg_C)

}