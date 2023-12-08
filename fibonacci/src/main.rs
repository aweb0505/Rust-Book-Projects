use std::io;

fn main() {
    //Simple program that computes the nth fibonacci number
    loop{
        println!("Enter the nth fibonacci number you want to compute.");

        let mut nth_fibonacci_number = String::new();

        io::stdin().read_line(&mut nth_fibonacci_number)
            .expect("Failed to read line");

        let nth_fibonacci_number: u32 = match nth_fibonacci_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input");
                continue;
            },
        };

        println!("The {}th fibonacci number is {}", nth_fibonacci_number, fibonacci(nth_fibonacci_number));
    }
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    return fibonacci(n-1) + fibonacci(n-2);
}
