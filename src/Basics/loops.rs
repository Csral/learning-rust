fn main() {

    loop {
        println!("Again!");
        break;
    };

    /* Actual looping! */

    let mut counter: i8 = 0;

    let result: i32 = loop {
        
        counter += 1;

        if counter >= 10 {
            break (counter * 2).into(); //* Break with specific value! */
        }

        //* .into() -> convert to required return type? */

    };

    println!("The result obtained was: {result}");

    /* You can specify labels during break to break specific labels! */

    counter = 0;
    let mut inner_counter = 0;

    'testing_break_: loop {

        inner_counter = 0;

        loop {

            if inner_counter >= 1 {
                break;
            }

            if counter >= 2 {
                break 'testing_break_;
            }

            inner_counter += 1;

        }

        counter += 1;

    };

    println!("Counter: {counter}\nInner counter> {inner_counter}");

    /* can i get returns then? */

    let result: i32 = 'testing_break_: loop {

        inner_counter = 0;

        loop {

            if inner_counter >= 1 {
                break;
            }

            if counter >= 2 {
                break 'testing_break_ (counter*2).into();
            }

            inner_counter += 1;

        }

        counter += 1;

    };

    println!("The result obtained was: {result}");

    //* YES WE CAN DO THAT! */

    let mut index: usize = 0;
    let arr: [i32; 5] = [0,1,2,3,4];
    
    while index < 5 {

        println!("The item at index {index} is {}", arr[index]);
        index += 1;

    };

    println!("Same thing using a for loop: ");

    for element in arr {
        println!("Element is {element}");
    }

    for num in (0..4).rev() {
        println!("Number: {num}");
    }

    println!("Terminated!");

}