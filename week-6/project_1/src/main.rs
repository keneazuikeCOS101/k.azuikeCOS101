//Rust Program for a resturant menu
use std::io

fn main() {
    //Friendly Welcome of the Customer
    println!("Hello Customer. Welcome to our Resturant!\n");
    println!(" Here is our menu:");
    println!("    1. Pounded Yam / Edinkaiko Soup : ₦3,200
    2. Fried Rice & Chicken : ₦3,000
    3. Amala & Ewedu Soup : ₦2,500 
    4. Eba & Egusi Soup : ₦,2000
    5. White Rice & Stew : ₦2,500");
    println!("What would you like to get today(Please choose from 1 to 5)?");

    //Accepting the customer input
    let mut order1 = String::new();
    io::stdin().read_line(&str order1).trim().expect("Please select from 1 to 5")

    if order1 != "1" && order1 != "2" && order1 != "3" && order1 != "4" && order1 != "5" {
        println("Please choose from 1 to 5");
        io::stdin().read_line(&str order1).trim().expect("Please select from 1 to 5");
    } else{
        if order1 == "1"{
            println!("You have chosen Pounded Yam / Edinkaiko soup for ₦3,200")

        }
    }
}
