mod calc; 
use std::io::stdin;

fn main() { 
    // variable declarations
    let mut first_string = String::new(); 
    let mut second_string = String::new(); 
    let mut operator = String::new(); 
    let first_number: i32; 
    let second_number: i32;

    println!("Enter your first number: ");
    stdin().read_line(&mut first_string).expect("Could not read first number, sorry!"); 
    
    println!("Enter your second number: "); 
    stdin().read_line(&mut second_string).expect("Could not read second number, sorry!"); 
    
    first_number = first_string.trim().parse().unwrap();
    second_number = second_string.trim().parse().unwrap(); 

    println!("Enter your operator\nAvailable operators: + - / *"); 
    stdin().read_line(&mut operator).expect("Could not read operator, sorry!"); 


    if operator.contains("+") { 
        println!("{}", calc::add(first_number.into(), second_number.into())); 
    } else if operator.contains("-") { 
        println!("{}", calc::subtract(first_number.into(), second_number.into())); 
    } else if operator.contains("/") {
        println!("{}", calc::divide(first_number.into(), second_number.into()));
    } else if operator.contains("*") {
        println!("{}", calc::multiply(first_number.into(), second_number.into()));
    } else {
        println!("Operator {} not found.", operator); 
    }
}
