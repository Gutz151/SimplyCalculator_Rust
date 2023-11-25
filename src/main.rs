use std::io;

fn main() {
    println!("Enter the first number");

    //Creating the first empty string to save the entry of the user
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Please enter a valid input");

    //Convert the string to float and save that in 'input1'
    let num1: f64 = input1.trim().parse().expect("Please enter a valid number!");

    println!("Enter the second number");

    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Please enter a valid input");

    let num2: f64 = input2.trim().parse().expect("Please enter a valid number!");

    println!("Select an operation!");
    println!("1. Add");
    println!("2. Sub");
    println!("3. Mul");
    println!("4. Div");
    println!("");

    //Create a new empty string to save the user entry
    let mut operation = String::new();

    io::stdin().read_line(&mut operation).expect("Error in the entry :(");

    let operation: u32 = operation.trim().parse().expect("Please enter a valid number!");

    match operation {
        1 => println!("The total is: {}" , num1 + num2),
        2 => println!("The total is: {}", num1 - num2),
        3 => println!("The total is: {}", num1 * num2),
        4 => println!("The total is: {}", num1 / num2),
        _ => println!("Invalid option"),
        }

}
