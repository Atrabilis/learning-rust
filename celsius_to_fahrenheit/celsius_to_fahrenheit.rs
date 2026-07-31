fn celsius_to_fahrenheit(celsius:f64) -> f64{
    celsius * 9.0 / 5.0 + 32.0
}

fn main() {
    let celsius = 37.0;
    let fahrenheit = celsius_to_fahrenheit(celsius);
    println!("{}°C is equal to {}°F", celsius, fahrenheit);
}