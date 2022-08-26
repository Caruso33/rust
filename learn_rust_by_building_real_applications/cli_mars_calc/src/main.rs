use std;

fn main() {
    let mut input = String::new();
    println!("Enter your weight (kg):");
    std::io::stdin().read_line(&mut input).unwrap();

    let weight = input.trim();
    dbg!(weight);

    let earth_weight: f32 = weight.parse().unwrap();
    // let earth_weight: f32 = weight.parse::<f32>().unwrap();
    let mars_weight = calculate_weight_on_mars(&earth_weight);

    println!("Weight on Mars: {}kg", mars_weight);
}

fn calculate_weight_on_mars(weight: &f32) -> f32 {
    (weight / 9.81) * 3.711
}
