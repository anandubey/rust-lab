fn main() {
    let numbers: [i8; 15] = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3, 5];
    let mut max: i8 = numbers[0];
    let mut min: i8 = numbers[0];
    let mean: f64;

    let mut sum: f64 = 0.0;
    for num in numbers {
        if num > max {
            max = num;
        }
        if num < min {
            min = num;
        }
        sum = sum + (num as f64);
    }

    println!("{}", sum);
    mean = sum / (numbers.len() as f64);
    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.0);
    println!("Tests passed!");
}
