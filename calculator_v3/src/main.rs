// Caculator V3

use calculator_v3::calculadora;

fn main() {
    println!("Sum: {}",calculadora::sum::sum(2.0, 4.0));
    println!("Sum Int: {}",calculadora::sum::sum_int::sum(1, 2));
    println!("Subtraction: {}",calculadora::subtraction::subtraction(4.0, 2.0));
    println!("Multiplication: {}",calculadora::multiplication::multiplication(4.0, 2.0));
    println!("Division: {:?}",calculadora::division::division(4.0, 2.0));
    println!("Division Zero: {:?}",calculadora::division::division(4.0, 0.0));
}