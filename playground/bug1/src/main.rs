use std::io;

fn main() {
    println!("Enter a number: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input);

    let num: i32 = input.trim().parse().unwrap();

    let result = divide_by_two(num);

    println!("Half of your number is: {}", result);
}

fn divide_by_two(n: i32) -> i32 {
    if n % 2 != 0 {
        panic!("Cannot divide an odd number by two!");
    }
    n / 2
}
