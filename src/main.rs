fn main() {
    let x = 0.0;
    println!("{} °C is {} °F", x, celsius_to_fahrenheit(x));
    println!("{} °F is {} °C", x, fahrenheit_to_celsius(x));

    let x = 100.0;
    println!("{} °C is {} °F", x, celsius_to_fahrenheit(x));
    println!("{} °F is {} °C", x, fahrenheit_to_celsius(x));
}

fn celsius_to_fahrenheit(celcius: f32) -> f32 {
    9.0 / 5.0 * celcius + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    5.0 / 9.0 * (fahrenheit - 32.0)
}
