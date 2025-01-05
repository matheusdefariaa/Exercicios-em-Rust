use std::io;
use num_traits::pow;
fn main() {
    let mut number_one = String::new();
    let mut number_two = String::new();
    let mut number_three = String::new();
    
    println!("Digite um número inteiro");
    io::stdin()
    .read_line(&mut number_one)
    .expect("Erro de leitura");
    
    println!("Digite outro número inteiro");
    io::stdin()
    .read_line(&mut number_two)
    .expect("Erro de leitura");
    
    println!("Digite um número real");
    io::stdin()
    .read_line(&mut number_three)
    .expect("Erro de leitura");

    let number_one = number_one.trim().parse::<i32>().expect("Erro ao converter número");
    let number_two = number_two.trim().parse::<i32>().expect("Erro ao converter número");
    let number_three = number_three.trim().parse::<f64>().expect("Erro ao converter número");

    let res_one = pow(number_one,2) * (number_two / 2);
    let res_two = pow(number_one,3) as f64 + number_three;
    let res_three = pow(number_three,3);
    
    println!("R1 {res_one}");
    println!("R2 {res_two}");
    println!("R3 {res_three}");
}
