use std::io;

fn fizzbuzz(n: i32) -> String {
    if n % 3 == 0 && n % 5 == 0 {
        "FizzBuzz".to_string()
    } else if n % 3 == 0 {
        "Fizz".to_string()
    } else if n % 5 == 0 {
        "Buzz".to_string()
    } else {
        n.to_string()
    }
}

fn main() {
    loop {
        let mut input = String::new();
        
        println!("Enter a number (or 'q' to quit, 'help'): ");
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
            
        let command = input.trim();
            
        if command.eq_ignore_ascii_case("q") {
            break;
        }
        
        if command == "help" {
            println!("FizzBuzz is a programming exercise where you iterate
                    through a sequence of numbers (usually 1 to n) and
                    apply rules:\n\nIf a number is divisible by 3 -> Output
                    Fizz\nIf divisible by 5 -> Output Buzz\nIf a number is
                    divisible by both 3 and 5 -> Output FizzBuzz\nOtherwise ->
                    Output the number itself.");
            
            continue;
        }
            
            
        let number: i32 = match command.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };
        
        for i in 1..=number {
            println!("{}: {}", i, fizzbuzz(i));
        }
    }
}
