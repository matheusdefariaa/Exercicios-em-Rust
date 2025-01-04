use std::io;

fn main() {
    let mut radius = String::new();
    println!("Digite o raio do circulo");

    io::stdin()
        .read_line(&mut radius)
        .expect("Failed to read line");

    let radius: f64 = radius.trim().parse().expect("error converting type");

    let area = num_traits::pow(radius,2) * std::f64::consts::PI;
    println!("Área do círculo: {}",area);
}
