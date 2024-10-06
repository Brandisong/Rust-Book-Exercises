fn main() {
    let temp_f = 100.0;
    let temp_c = fahrenheit_to_celsius(temp_f);
    println!("{temp_f}°F is {temp_c}°C");
}

fn fahrenheit_to_celsius(temp: f32) -> f32 {
    (temp - 32.0) * 5.0/9.0
}