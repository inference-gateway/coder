use std::io;

fn main() {
    println!("Welcome to the number divider!\nPlease enter an even number: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let num: i32 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("\nPlease enter a valid number.");
            return;
        }
    };

    let result = match divide_by_two(num) {
        Ok(r) => r,
        Err(e) => {
            println!("\n{}", e);
            return;
        }
    };

    println!("\nHalf of your number is: {}", result);
    println!("\nThank you for using the number divider!");
}

fn divide_by_two(n: i32) -> Result<i32, String> {
    if n % 2 != 0 {
        return Err("Cannot divide an odd number by two! Please try an even number.".to_string());
    }
    Ok(n / 2)
}