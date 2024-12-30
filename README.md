# Simple Calculator Program in Rust

## Overview

This is a simple calculator program built using Rust. It allows users to perform basic arithmetic operations such as addition, subtraction, multiplication, and division. The program provides a user-friendly command-line interface for interacting with the calculator. Users can input two numbers and choose an operation to perform on them.

## Features

- Supports four operations:
  - Addition
  - Subtraction
  - Multiplication
  - Division
- Prompts the user for input and handles invalid entries
- Continues to prompt for new operations until the user chooses to exit
- Provides clear error handling for division by zero

## To use this program

### Prerequisites

- Make sure you have rust and cargo installed. You can follow the installation instructions at [Rust Official Installation Guide](https://www.rust-lang.org/tools/install).

### Running the program

1. Clone the repository:

```sh
git clone https://github.com/Obcodelab/Calculator.git
```

2. Navigate to the project directory:

```sh
cd Calculator
```

3. Run the program:

```sh
cargo run
```

### How it works

1. The program prompts the user to select an operation (Add, Subtract, Multiply, Divide).
2. After selecting an operation, it asks for two numbers as input.
3. It performs the selected operation and displays the result.
4. The user is then asked if they want to perform another operation. If yes, the process repeats; otherwise, the program ends.

### Code Explanation

- The program defines a Calculator struct with two fields: first_number and second_number, representing the numbers on which the operations will be performed.
- The Calculator struct has methods for each operation (add, subtract, multiply, divide), which are implemented as part of the struct’s functionality.
- Input handling is done through the input function, which prompts the user and reads the input. The input is then parsed into integers using the number_check function, which ensures valid numerical input.
- Error handling is included for division by zero, and invalid inputs are handled by asking the user to re-enter the values.

### Contributing

Feel free to contribute to this project! If you find any bugs or want to suggest improvements, please create an issue or submit a pull request.
