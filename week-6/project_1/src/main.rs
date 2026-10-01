//Rust Program for a resturant menu
use std::io;
use std::process;

fn main() {
    //Friendly Welcome of the Customer
    println!("\nHello Customer. Welcome to our Resturant!\n");
    println!(" Here is our menu:");
    println!("    1. Pounded Yam / Edinkaiko Soup : ₦3,200
    2. Fried Rice & Chicken : ₦3,000
    3. Amala & Ewedu Soup : ₦2,500 
    4. Eba & Egusi Soup : ₦2,000
    5. White Rice & Stew : ₦2,500");
    println!("What would you like to get today(Please choose from 1 to 5)?");

    //Accepting the customer input
    let mut input1 = String::new();
    let mut amount = 0;
    io::stdin().read_line(&mut input1).expect("Please select from 1 to 5");
    let order1:i32 = input1.trim().parse().expect("Please select from 1 to 5");
    

    if order1 < 1 || order1 > 5 {
        println!("Please choose from 1 to 5");
        process::exit(1);
    } else { 
            if order1 == 1 {
                println!("You have chosen Pounded Yam / Edinkaiko soup for ₦3,200\n");
                println!("How many servings would you like?");
                let mut input2 = String::new();
                io::stdin().read_line(&mut input2).expect("Please select a valid number");
                let quantity:i32 = input2.trim().parse().expect("Please select a valid number");
                amount = quantity * 3200;
                println!("You have chosen {} servings of Pounded Yam / Edinkaiko soup for ₦{}\n", quantity, amount);
            }else if order1 == 2 {
                println!("You have chosen Fried rice & chicken for ₦3,000\n");
                println!("How many servings would you like?");
                let mut input2 = String::new();
                io::stdin().read_line(&mut input2).expect("Please select a valid number");
                let quantity:i32 = input2.trim().parse().expect("Please select a valid number");
                amount = quantity * 3000;
                println!("You have chosen {} servings of Fried rice and Chicken for ₦{}\n", quantity, amount);
            }else if order1 == 3 { 
                println!("You have chosen Amala & Ewedu soup for ₦2,500\n");
                println!("How many servings would you like?");
                let mut input2 = String::new();
                io::stdin().read_line(&mut input2).expect("Please select a valid number");
                let quantity:i32 = input2.trim().parse().expect("Please select a valid number");
                amount = quantity * 2500;
                println!("You have chosen {} servings of Amala & Ewedu Soup for ₦{}\n", quantity, amount);
            }else if order1 == 4 {
                println!("You have chosen Eba & Egusi Soup for ₦2,000\n");
                println!("How many servings would you like?");
                let mut input2 = String::new();
                io::stdin().read_line(&mut input2).expect("Please select a valid number");
                let quantity:i32 = input2.trim().parse().expect("Please select a valid number");
                amount = quantity * 2000;
                println!("You have chosen {} servings of Eba & Egusi Soup for ₦{}\n", quantity, amount);
            }else if order1 == 5 {
               println!("You have chosen Whit Rice & Stew for ₦2,500\n");
                println!("How many servings would you like?");
                let mut input2 = String::new();
                io::stdin().read_line(&mut input2).expect("Please select a valid number");
                let quantity:i32 = input2.trim().parse().expect("Please select a valid number");
                amount = quantity * 2500;
                println!("You have chosen {} servings of White Rice & Stew for ₦{}\n", quantity, amount);
            }
            
    }
    //Determining eligibility for discount
    if amount > 10000 {
        let bill = (amount * 19) / 20;
        println!("Your total bill is greater than ₦10,000 and you get a 5% discount.");
        println!("Your total bill is ₦{}", bill);
    }else{
        println!("Your total bill is ₦{}", amount);
    }
}