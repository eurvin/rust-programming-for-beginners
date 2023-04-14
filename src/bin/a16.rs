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
    let students = vec![
        Student {
            name: "John".to_owned(),
            locker: Some(5),
        },
        Student {
            name: "Alice".to_owned(),
            locker: None,
        },
        Student {
            name: "Bob".to_owned(),
            locker: Some(1),
        },
    ];
    for student in students {
        match student.locker {
            Some(number) => println!("{:?} has number {:?}", student.name, number),
            None => println!("the student does not have a locker"),
        }
    }
}
