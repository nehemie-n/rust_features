use core::fmt::Debug;

/// Represents a worker with an age and a name.
#[derive(Clone)]
struct Worker {
    age: u16,
    name: String,
}

impl Debug for Worker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return f.write_str(format!("Name: {}, age: {}", &self.name, &self.age).as_str());
    }
}

/// Represents a manager with an age, a name, and a list of subordinates.
#[derive(Clone)]
struct Manager {
    age: u16,
    name: String,
    subordinates: Vec<Employee>,
}

impl Debug for Manager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return f.write_str(
            format!(
                "Name: {}, age:{}, subordinates:{}",
                &self.name,
                &self.age,
                &self.subordinates.len()
            )
            .as_str(),
        );
    }
}

/// Represents different types of employees - either a manager or a worker.
#[derive(Debug, Clone)]
enum Employee {
    Manager(Manager),
    Worker(Worker),
}

/// Simulates employee interactions and debugging output.
pub fn simulate() {
    let kim: Employee = Employee::Worker(Worker {
        age: 16,
        name: String::from("Kim"),
    });
    let akim = Employee::Worker(Worker {
        age: 16,
        name: String::from("Akim"),
    });
    let osanjo: Employee = Employee::Manager(Manager {
        age: 16,
        name: String::from("osanjo"),
        subordinates: vec![kim.clone(), akim.clone()],
    });

    println!("Osanjo : {:?}", osanjo);

    for employee in [akim, kim, osanjo] {
        match employee {
            Employee::Worker(worker) => {
                println!("Worker: {:?}", worker);
            }
            Employee::Manager(manager) => {
                println!("Manager: {:?}", manager);
            }
        }
    }
}
