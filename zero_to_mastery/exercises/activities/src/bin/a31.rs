// Topic: Trait Objects
//
// Summary:
//   A contractor wants a program that can sum the cost of materials based
//   on how many square meters are required for a job.
//
// Requirements:
// * Calculate multiple material types with different costs
// * Must be able to process a list of varying materials
// * Material types and cost includes:
//   * Carpet - $10 per square meter
//   * Tile - $15 per square meter
//   * Wood - $20 per square meter
// * Square meters must be taken into account
//
// Notes:
// * Create a trait that can be used to retrieve the cost of a material
// * Create trait objects and store them in a vector for processing
// * Use a function to calculate the total cost
// * Process at least 3 different materials

trait MaterialCost {
    fn cost_per_sqm(&self) -> f64;
    fn cost_of_material(&self, sqm: &f64) -> f64 {
        sqm * self.cost_per_sqm()
    }
}

fn total_cost(materials: &Vec<Box<dyn MaterialCost>>, sqm: &f64) -> f64 {
    materials.iter().map(|mat| mat.cost_of_material(sqm)).sum()
}

struct Carpet(f64);
impl MaterialCost for Carpet {
    fn cost_per_sqm(&self) -> f64 {
        self.0
    }
}

struct Tile(f64);
impl MaterialCost for Tile {
    fn cost_per_sqm(&self) -> f64 {
        self.0
    }
}

struct Wood(f64);
impl MaterialCost for Wood {
    fn cost_per_sqm(&self) -> f64 {
        self.0
    }
}

fn main() {
    let sqm: f64 = 10.;

    let mut materials: Vec<Box<dyn MaterialCost>> = vec![];

    materials.push(Box::new(Carpet(10.0)));
    materials.push(Box::new(Tile(15.0)));
    materials.push(Box::new(Wood(20.0)));

    let costs = total_cost(&materials, &sqm);
    println!("Total costs for {:?} sqm is {:?}", sqm, costs);
}
