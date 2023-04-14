// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

struct Customer {
    age: i32,
}

#[derive(Debug)]
enum HasAccess {
    Restricted,
    Allowed,
}

fn get_access(customer: &Customer) -> Result<HasAccess, HasAccess> {
    let is_restricted = customer.age >= 21;
    match is_restricted {
        true => Ok(HasAccess::Allowed),
        false => Err(HasAccess::Restricted),
    }
}

fn print_access(access: &HasAccess) {
    println!("the customer is {:?}", access);
}

fn has_access(customer: &Customer) -> Result<(), HasAccess> {
    let has_access: HasAccess = get_access(&customer)?;
    print_access(&has_access);
    Ok(())
}

fn main() {
    let customer = Customer { age: 20 };
    let access = has_access(&customer);
    println!("The customer is not {:?}", access);
}
