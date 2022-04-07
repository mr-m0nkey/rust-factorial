use std::io;

fn main() {
    println!("Enter a number");

    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("failed to read from stdin");

    let number = number.trim();
    let number = number.parse::<i32>().expect("Invalid input");

    println!("The factorial of {} is {}", number, factorial(number));
    

}

fn factorial(number: i32) -> i32 {
    if number <= 1 {
        return 1
    }

    number * factorial(number - 1)
}
