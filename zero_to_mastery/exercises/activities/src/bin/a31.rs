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
    fn cost_of_material(&self, sqm: &f64) -> f64;
}

struct Carpet {
    cost_per_sqm: f64,
}

impl MaterialCost for Carpet {
    fn cost_of_material(&self, sqm: &f64) -> f64 {
        sqm * self.cost_per_sqm
    }
}

struct Tile {
    cost_per_sqm: f64,
}

impl MaterialCost for Tile {
    fn cost_of_material(&self, sqm: &f64) -> f64 {
        sqm * self.cost_per_sqm
    }
}

struct Wood {
    cost_per_sqm: f64,
}

impl MaterialCost for Wood {
    fn cost_of_material(&self, sqm: &f64) -> f64 {
        sqm * self.cost_per_sqm
    }
}

fn print_cost_of_material(material: impl MaterialCost, sqm: &f64) {
    println!("{:?}", material.cost_of_material(sqm));
}

fn main() {
    let sqm: f64 = 10.;

    let carpet = Carpet { cost_per_sqm: 10.0 };
    let tile = Tile { cost_per_sqm: 15.0 };
    let wood = Wood { cost_per_sqm: 20.0 };

    print_cost_of_material(carpet, &sqm);
    print_cost_of_material(tile, &sqm);
    print_cost_of_material(wood, &sqm);
}
