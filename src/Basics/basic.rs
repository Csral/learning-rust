fn main() {
    println!("Hello world!");

    /*
    
        * Early returns are supported by using 'return' keyword.
        * If statements do not support any data type but bool. So things like " if (number)"" won't work and we must use "if number != 0"

        * No '?' operator, follow: "let number = if condition { 5 } else { "six" };"

    */

    let x : u32 = 12_576;
    println!("X: {}", x);

    let tuple_w_annotation: (i32, u32, i64, char) = (-100, 350, 2_929, 'A');
    let tuple_wo_annotation = (150, "tes");

    let (a,b,c,d) = tuple_w_annotation;
    println!("a: {}, b: {}, c: {}, d: {}", a,b,c,d);
    
    let (a,b) = tuple_wo_annotation;

    println!("a: {}, b: {}", a,b);

    println!("Accessing elements of tuple by '.' pointer -> 1st element: {}, 2nd element: {}", tuple_w_annotation.1, tuple_w_annotation.2);

}