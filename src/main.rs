use std::io;

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::Write::flush(&mut io::stdout()).expect("Failed to flush stdout");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    user_input.trim().to_string()
}

fn number_check(user_input: &str) -> i32 {
    match user_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            0
        }
    }
}
fn main() {
    struct Calculator {
        first_number: i32,
        second_number: i32,
    }
    impl Calculator {
        fn add(&self) -> i32 {
            self.first_number + self.second_number
        }
        fn subtract(&self) -> i32 {
            self.first_number - self.second_number
        }
        fn multiply(&self) -> i32 {
            self.first_number * self.second_number
        }
        fn divide(&self) -> f32 {
            let first_number = self.first_number as f32;
            let second_number = self.second_number as f32;
            if second_number != 0.0 {
                first_number / second_number
            } else {
                println!("Division by zero is not allowed");
                0.0
            }
        }
    }
    println!("This is a calculator that does the function below with two numbers");
    println!(
        "
         ---------------
        | 1. Add        |
        | 2. Subtract   |
        | 3. Multiply   |
        | 4. Divide     |
         ---------------
        "
    );
    let mut active: bool = true;
    while active {
        let action_number = input("Enter the operation number you want to perform (1 - 4): ");
        let action_number = number_check(&action_number);
        let action_list = [1, 2, 3, 4];
        if action_list.contains(&action_number) {
            let first_number = input("Enter the first number: ");
            let first_number = number_check(&first_number);
            let second_number = input("Enter the second number: ");
            let second_number = number_check(&second_number);
            let calculator = Calculator {
                first_number,
                second_number,
            };
            match action_number {
                1 => println!(
                    "The sum of {} and {} is {}",
                    first_number,
                    second_number,
                    calculator.add()
                ),
                2 => println!(
                    "The difference between {} and {} is {}",
                    first_number,
                    second_number,
                    calculator.subtract()
                ),
                3 => println!(
                    "The product of {} and {} is {}",
                    first_number,
                    second_number,
                    calculator.multiply()
                ),
                4 => println!(
                    "The division of {} and {} is {}",
                    first_number,
                    second_number,
                    calculator.divide()
                ),
                _ => println!("Please enter a valid number"),
            }
            let response = input("Do you want to perform another operation? (yes/no): ");
            if response == "no" {
                active = false;
            }
        } else {
            println!("Please enter a valid number");
        }
    }

    println!("Thank you for using the calculator");
}
