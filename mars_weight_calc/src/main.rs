use std::io;

fn main() {
    println!("Enter your weight (kg): ");
    let mut read_buf = String::new();

    io::stdin().read_line(&mut read_buf).unwrap();

    let earth_weight: f32 = read_buf.trim().parse().unwrap();
    let mars_weight = calc_weight_on_mars(earth_weight);

    print!("Weight on Mars: {}kg", mars_weight);
}

fn calc_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

