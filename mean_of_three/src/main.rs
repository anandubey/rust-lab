fn main() {
    let num_1 = 13;
    let num_2 = 2.3;
    let num_3: f32 = 120.0;
    
    let result = (num_1 as f64 + num_2 + num_3 as f64) / 3.0;
    //println!("Mean of three numbers {0}, {1}, {2} is {3}", num_1, num_2, num_3, result);
    assert_eq!(result, 45.1);

    println!("Test Passed!");
}
