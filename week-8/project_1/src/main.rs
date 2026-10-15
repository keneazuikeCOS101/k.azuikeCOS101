//Rust program to check APS level

use std::io;

fn main () {
    println!("\nWelcome to the APS Level Checker.");

    let office_admin = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];
    let academic = vec!["None of the options", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
    let lawyer = vec!["Paralegal", "Junior Assosciate", "Associate", "Senior Associate (1-2 years)","Senior Associate (3-4 years)", "Partner"];
    let teacher = vec!["Placement", "Classroom Teacher", "Senior teacher", "Leading Teacher", "Deputy principal", "Principal"];
    let public_servant = vec!["APS 1-2", "APS 3-4", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

    

    //find out user field
    loop{
        println!("\nThese are the fields available:
        1. Office admin
        2. Academic
        3. Lawyer
        4. Teacher");
        

        let mut input1= String::new();
        println!("\nWhat field do you work in(Enter from 1 to 4)?");    
        io::stdin().read_line(&mut input1).expect("Failed to read input");
        let mut field: u8 = input1.trim().parse().expect("please enter a valid number from 1 to 4");
        
        //find out user job
        if field == 1{
            loop{
                println!("\nYou have chosen Office Admin.");
                for i in 0..office_admin.len(){
                    println!("{}. {}", i + 1, office_admin[i]);
                }
                println!("\nWhich of the above jobs do u do(Select from 1 to 6)?");

                let mut input2 = String::new();
                io::stdin().read_line(&mut input2).expect("Failed to read input");
                let mut job: usize = input2.trim().parse().expect("please enter a valid number from 1 to 6");

                if job < 1 || job > 6{
                    println!("\nPlease select from 1 to 6");
                }else{
                    println!("\nYour APS level is {}.", public_servant[job-1]);
                    break;
                }
            }
            break;
        } else if field == 2{
            loop{
                println!("\nYou have chosen Academic.");
                for i in 0..academic.len(){
                    println!("{}. {}", i + 1, academic[i]);
                }
                println!("\nWhich of the above jobs do u do?");

                let mut input2 = String::new();
                io::stdin().read_line(&mut input2).expect("Failed to read input");
                let mut job: usize = input2.trim().parse().expect("please enter a valid number from 1 to 6");

                if job < 1 || job > 6{
                    println!("\nPlease select from 1 to 6");
                }else{
                    println!("\nYour APS level is {}.", public_servant[job-1]);
                    break;
                }   
            } 
            break;
        }else if field == 3 {
            loop{
                println!("\nYou have chosen Lawyer.");
                for i in 0..lawyer.len(){
                    println!("{}. {}", i + 1, lawyer[i]);
                }
                println!("\nWhich of the above jobs do u do?");

                let mut input2 = String::new();
                io::stdin().read_line(&mut input2).expect("Failed to read input");
                let mut job: usize = input2.trim().parse().expect("please enter a valid number from 1 to 6");

                if job < 1 || job > 6{
                    println!("\nPlease select from 1 to 6");
                }else{
                    println!("\nYour APS level is {}.", public_servant[job-1]);
                    break;
                }
            }    
            break;
        }else if field == 4{
            loop{
                println!("\nYou have chosen Teacher.");
                for i in 0..teacher.len(){
                    println!("{}. {}", i + 1, teacher[i]);
                }
                println!("\nWhich of the above jobs do u do?");

                let mut input2 = String::new();
                io::stdin().read_line(&mut input2).expect("Failed to read input");
                let mut job: usize = input2.trim().parse().expect("please enter a valid number from 1 to 6");

                if job < 1 || job > 6{
                    println!("\nPlease select from 1 to 6");
                }else{
                    println!("\nYour APS level is {}.", public_servant[job-1]);
                    break;
                }
            }
            break;
        }else {
            println!("\nSelect from 1 to 4.");
        }
    }
}