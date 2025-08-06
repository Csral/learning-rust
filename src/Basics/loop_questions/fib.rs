use std::io;

fn main() {

    // 0 1 1 2 3 5

    let mut input: String = String::new();
    println!("Enter no. of Fibonacci numbers to display> ");
    io::stdin().read_line(&mut input).expect("Unable to read!");

    let input: usize = input.trim().parse().expect("NaN");
    let fib_series = fib_no(&input);

    for number in fib_series {
        print!("{number} ");
    }

    print!("\n");

}

fn fib_no(max: &usize) -> Vec<i32> {

    let mut ans = vec![0, 1];

    for i in 2..(*max) {
        ans.push(ans[i-1] + ans[i-2]);
    }

    ans

}