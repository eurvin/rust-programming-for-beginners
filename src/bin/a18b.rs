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

// * Use an enum to represent all types of employees
#[derive(Debug)]
enum Role {
    MaintenanceCrew,
    MarketingDepartment,
    Manager,
    LineSupervisor,
    KitchenStaff,
    AssemblyTechnician,
}

// * Use a struct to store the employee type and whether they are
//   still employed
enum Employment {
    Active,
    Terminated,
}

struct Employee {
    role: Role,
    employment: Employment,
}

// * Use a function that returns a Result to determine if the employee
//   may enter the building

fn get_role(employee: &Employee) -> Result<(), String> {
    match &employee.role {
        Role::MaintenanceCrew => Ok(()),
        Role::MarketingDepartment => Ok(()),
        Role::Manager => Ok(()),
        _ => Err("Wrong Role".to_owned()),
    }
}

fn try_access(employee: &Employee) -> Result<(), String> {
    match employee.employment {
        Employment::Terminated => return Err("Terminated".to_owned()),
        _ => (),
    }

    match employee.role {
        Role::MaintenanceCrew => Ok(()),
        Role::MarketingDepartment => Ok(()),
        Role::Manager => Ok(()),
        _ => return Err("Wrong Role".to_owned()),
    }
}

fn print_access(employee: &Employee) -> Result<(), String> {
    try_access(&employee)?;
    println!("auth ok");
    Ok(())
}

fn main() {
    let employee = Employee {
        role: Role::MaintenanceCrew,
        employment: Employment::Terminated,
    };
    match print_access(&employee) {
        Err(e) => println!("This employee does not have access to the building {:?}", e),
        _ => (),
    }
}
