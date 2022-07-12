use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use std::io::{self, Write};
use std::collections::HashMap;

use phf::{phf_map};

#[derive(Debug, EnumIter, Clone, Hash)]
enum Operation {
    Add,
    Remove,
    ListDepartment,
    ListAll,
    Quit,
}

static OPERATION_NUMBERS: phf::Map<&'static str, Operation> = phf_map! {
    "1" => Operation::Add,
    "2" => Operation::Remove,
    "3" => Operation::ListDepartment,
    "4" => Operation::ListAll,
    "5" => Operation::Quit,
};

static OPERATION_BANNERS: phf::Map<&'static str, &'static str> = phf_map! {
    "1" => "Add",
    "2" => "Remove Department",
    "3" => "List Members of Department",
    "4" => "List All Departments With Members",
    "5" => "Quit",
};

struct UserDepartment {
    user: String,
    department: String,
}

fn menu() {
    println!("\n--------------------------------");
    println!("Menu:");
    for (n, _) in Operation::iter().enumerate() {
        println!("{} - {}", n+1, OPERATION_BANNERS.get(&(n+1).to_string()).unwrap());
    }
    println!("--------------------------------");
}

fn read_input() -> String {
    print!("Insert Option:");
    io::stdout().flush().unwrap();
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string)
         .expect("Failed to read line");
    input_string.pop();
    input_string
}

fn read_user() -> String {
    print!("Insert User:");
    io::stdout().flush().unwrap();
    let mut input_user = String::new();
    io::stdin().read_line(&mut input_user)
         .expect("Failed to read user");
    input_user.pop();
    input_user
}

fn read_department() -> String {
   print!("Insert Department:");
    io::stdout().flush().unwrap();
    let mut input_department = String::new();
    io::stdin().read_line(&mut input_department)
        .expect("Failed to read department");
    input_department.pop();
    input_department
}

fn read_user_department() -> (String, String) {
    (read_user(), read_department())
}


fn process_list_all(h: &HashMap<String, Vec<String>>) -> Result<(), ()> {
    for (k, u) in h {
	println!("Department:{}->Members:{:?}", k, u);
    }
    Ok(())
}

fn prompt_user_and_department() -> UserDepartment {
    let (u, d) = read_user_department();
    UserDepartment{ user: u, department: d }
}

fn process_add(h: &mut HashMap<String, Vec<String>>) -> Result<(), ()> {
    let user_department = prompt_user_and_department();
    h.entry(user_department.department).or_insert(Vec::new()).push(user_department.user);
    Ok(())
}

fn process_list_department(h: &mut HashMap<String, Vec<String>>) -> Result<(), ()> {
    let department = read_department();
    match h.get(&department) {
        None => {
            println!("Department:{} does not exist !!!", department);
            Ok(())
        }
        Some(users) => {
            println!("{:?}", users);
            Ok(())
        }
    }
}

fn process_remove_department(h: &mut HashMap<String, Vec<String>>) -> Result<(), ()> {
    let department = read_department();
    match h.get(&department) {
         None => {
             println!("Department:{} does not exist !!!", department);
             Ok(())
         }
         Some(_) => {
             h.remove(&department);
             Ok(())
         }
    }
}

fn process_operation(o: Operation, h: &mut HashMap<String, Vec<String>>) -> Result<(), ()> {
    match o {
         Operation::Add => process_add(h),
         Operation::Remove => process_remove_department(h),
         Operation::ListDepartment => process_list_department(h),
         Operation::ListAll => process_list_all(h),
         _ => Err(()),
    }
}

fn read_operation() -> Operation {
    //let input_string = read_input();
    let op = OPERATION_NUMBERS.get(&read_input());
    match op {
        Some(operation) => operation.clone(),
        None => {
            println!("Invalid Option");
            read_operation()
        }
    }
}

fn operation_loop() {
    let mut user_hash: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        // 1 - Show menu
        menu();

        // 2 - Ask Operation
        let operation = read_operation();

        // 3 - Process Operation
        println!("{:?}:", operation);
        match operation {
            Operation::Quit => { break; },
            _ => { process_operation(operation, &mut user_hash).expect("Error on operation"); },
        }
    }
}

fn go() {
    // Operation loop
    operation_loop();
}

fn main() {
    go();
}
