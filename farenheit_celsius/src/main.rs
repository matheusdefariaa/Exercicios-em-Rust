use std::io;

fn main() {
    let mut temperature = String::new();

    println!("Enter the temperature in farenheit");

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    let temperature = temperature.trim().parse::<f64>().expect("Invalid entry");

    let celsius = 5.0 * (temperature - 32.0) / 9.0;

    println!("Farenheit: {temperature}\nCelsius: {celsius}");


}
