use std::io;

fn main() {
    let mut metros: String = String::new();
    println!("Digite o valor em metros");
    
    io::stdin()
    .read_line(&mut metros)
    .expect("Error");
    
    let metros: f64 = metros.trim().parse().expect("Error");
    
    let centimetros = metros * 100.0;
    println!("{} centimetros",centimetros)
}
