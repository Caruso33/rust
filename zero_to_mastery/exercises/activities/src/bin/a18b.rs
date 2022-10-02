// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

enum Position {
    Maintenance,
    Marketing,
    Manager,
    LineSupervisor,
    Kitchen,
    Assembly,
}

enum Status {
    Employed,
    NotEmployed,
}

struct Employee {
    position: Position,
    status: Status,
}

impl Employee {
    fn try_access(&self) -> Result<(), &str> {
        match self.position {
            Position::Maintenance | Position::Marketing | Position::Manager => match self.status {
                Status::Employed => Ok(()),
                _ => Err("Invalid status!"),
            },
            _ => Err("Invalid position!"),
        }
    }
}

fn main() {
    let ali = Employee {
        position: Position::Maintenance,
        status: Status::NotEmployed,
    };
    let bob = Employee {
        position: Position::Marketing,
        status: Status::Employed,
    };
    let charli = Employee {
        position: Position::LineSupervisor,
        status: Status::Employed,
    };
    let doris = Employee {
        position: Position::Kitchen,
        status: Status::NotEmployed,
    };

    print_access(&ali);
    print_access(&bob);
    print_access(&charli);
    print_access(&doris);
}

fn print_access(e: &Employee) -> Result<(), &str> {
    let attempt_success = e.try_access()?;
    println!("All good, go ahead");
    Ok(())
}
