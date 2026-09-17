//Rust program to take an employee's age and experience and
//to give annual incentive

use std::io;

fn main(){

    let mut input1 = String::new();
    let mut input2 = String::new();
    
    //employee enters his/her experience level
    println!("Enter your experience level(experienced or no experience)");
    io::stdin().read_line(&mut input1).expect("Invalid input");
    let experience = input1.trim().to_lowercase();

    //employee enters his/her age
    println!("Enter your age.");
    io::stdin().read_line(&mut input2).expect("Invalid input");
    let age:u8 = input2.trim().parse().expect("Not a valid number");

    //determination of annual incentive
    if experience == "experienced"{
        if age >= 40{
            println!("Your annual incentive is 1,560,000");
        } else if 30 <= age && age <= 39{
            println!("Your annual incentive is 1,480,000");
        } else if age <= 29{
            println!("Your annual incentive is 1,380,000");
        }
    }else if experience == "no experience"{
        println!("Your annual incentive is 100,000")
    }else {
        println!("invalid entry.Enter either experienced or no experience")
    }



}