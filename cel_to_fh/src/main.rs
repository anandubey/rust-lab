fn celsius_to_fahrenheit(deg_cel: f32) -> f32 {
    (deg_cel * 1.8) + 32.0
}

fn main() {
    let deg_cel = 23.0;
    let deg_fh = celsius_to_fahrenheit(deg_cel);
    println!("Temp in fahrenheit is {}", deg_fh);
}
