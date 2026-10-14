//Rust program for EY interview

use std::io;

fn main(){
    println!("Welcome to the Ernst & Young Global Ltd. Interview");

    let mut names :Vec<String> = Vec::new();
    let mut years :Vec<usize> = Vec::new();
    let mut data :Vec<(String, usize)> = Vec::new();
    
    //loop to accept the applicants input
    loop{
        let mut input1 = String::new();
        println!("\nAre you an applicant?");
        io::stdin().read_line(&mut input1).expect("Failed to read input");
        let option = input1.trim().to_lowercase();
        
        if option == "yes"{
            let mut input2= String::new();
            println!("\nPlease, what is your name?");
            io::stdin().read_line(&mut input2).expect("Failed to read your name");
            let name:String = input2.trim().parse().expect("please enter a valid name.");
            names.push(name.clone());

            let mut input3= String::new();
            println!("Please, how many years have you worked for?");
            io::stdin().read_line(&mut input3).expect("Failed to read your age");
            let experience:usize = input3.trim().parse().expect("please enter a valid age");
            years.push(experience);

            data.push((name, experience));
        } else if option == "no" {
            break;
        } else{
            println!("Please enter yes or no");
        }
    }
    //checks for maximum years of experience
    let mut max:usize = years[0];
    let mut index:usize = 0;
    for i in 0..years.len() {
        if years[i] > max{
            max = years[i];
            index = i;
        }
    }
    //prints output
    println!("\n{} is the one with the most experience with {} years of experience", names[index], max);
}