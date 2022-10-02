// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let student1 = Student {
        name: "Ali".to_string(),
        locker: None,
    };
    let student2 = Student {
        name: "Bob".to_string(),
        locker: Some(1),
    };
    let student3 = Student {
        name: "Charli".to_string(),
        locker: Some(2),
    };

    let students = vec![student1, student2, student3];

    for s in students {
        match s.locker {
            Some(id) => println!("{:?} has locker id: {:?}", s.name, id),
            None => println!("{:?} has no Locker", s.name),
        }
    }
}
