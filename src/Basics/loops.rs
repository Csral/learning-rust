fn main() {

    loop {
        println!("Again!");
        break;
    };

    /* Actual looping! */

    let counter: i8 = 0;

    let result: i32 = loop {

        counter += 1;

    };

}