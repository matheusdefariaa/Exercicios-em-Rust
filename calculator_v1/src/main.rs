use std::{io,process};

fn main() {
    let mut number_one = String::new();
    let mut number_two = String::new();

    println!("Calculator V1");

    loop {
        let opc = menu();

        println!("Enter the first number");

        io::stdin()
            .read_line(&mut number_one)
            .expect("Failed to read line");

        println!("Enter the second number");

        io::stdin()
            .read_line(&mut number_two)
            .expect("Failed to read line");

        let number_one: f64 = match number_one.trim().parse() {
            Ok(e) => e,
            Err(_) => continue,
        };

        let number_two: f64 = match number_two.trim().parse() {
            Ok(e) => e,
            Err(_) => continue,
        };
    
        match opc {
            1 => {
                sum(number_one, number_two);
                break;
            }
            2 => { 
                subtraction(number_one, number_two);
                break;
            }
            3 => {
                multiplication(number_one, number_two);
                break;
            }
            4 => {
                division(number_one, number_two);
                break;
            }
            0 => process::exit(1),
            _ => continue,
        };
    }
}

fn menu() -> u8 {
    let mut opc = String::new();

    println!("1 - Sum");
    println!("2 - Subtraction");
    println!("3 - Multiplication");
    println!("4 - Division");
    println!("0 - Exit");

    loop{
        io::stdin()
            .read_line(&mut opc)
            .expect("Failed to read line");

        let _opc: u8 = match opc.trim().parse() {
            Ok(e) => {
                return e;
            },

            Err(_) => {
                println!("Invalid Entry!");
                opc.clear();
                continue;
            }
        };
    }
}

fn sum(n1: f64,n2: f64) {
    println!("Sum: {n1} + {n2} = {}", n1 + n2);

}

fn subtraction(n1: f64,n2: f64) {
    println!("Subtraction: {n1} - {n2} = {}", n1 - n2);

}

fn multiplication(n1: f64, n2: f64) {
    println!("Multiplication: {n1} * {n2} = {}", n1 * n2);
}

fn division(n1:f64,n2:f64) {
    if n2 == 0.0 {
        println!("Division by Zero");
    }

    else {
        println!("Division: {n1} / {n2} = {}", n1 / n2);
    }
}
