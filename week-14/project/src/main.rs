//Globacom database rust program
use std::io;
use std::fs::File;
use std::io::Read;

fn extract_db(x :usize){ // function to read database contents to a string
    let filenames :[&str;5] = ["globacom.sql", "project.sql", "staff.sql", "customer.sql", "vendor.sql"];
    let mut file = File::open(filenames[x-1]).expect("Failed to get file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read to string");
    println!("{}", contents);
}

fn main() {
    println!("\nWelcome to Globacom Ltd.");//introducing user
    let personnel :[&str; 5] = ["Administrator", "Project Manager", "Employee", "Customer", "Vendor"];    
    println!("\nThese are the following catergories in Globacom:");
    for i in 0..personnel.len(){//printing out available categories
        println!("{}. {}", i+1, personnel[i]);
    }
    println!("\nWhich of the following categories do you belong to?");
    
    loop{
        let mut input1 = String::new();//collecting user input
        io::stdin().read_line(&mut input1).expect("Failed to read");
        let option :usize = input1.trim().parse().expect("Invalid input");
        
        if option <= 5 && option >=1{// deciding the file which will be read
            extract_db(option);
            break;
        }else{ //if user faults in providing correct input
            println!("Enter a number between 1 and 5");
        }
    }
}
