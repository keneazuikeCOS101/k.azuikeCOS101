use std::io;
use std::f64::consts::PI;


// function for trapezium area
fn trapezium(){
    println!("\nEnter the height of the trapezium:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Please enter a valid number");
    let height :f64 = input1.trim().parse().expect("Enter a valid number!");

    println!("\nEnter the first base of the trapezium:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Please enter a valid number");
    let base1 :f64 = input2.trim().parse().expect("Enter a valid number!");

    println!("\nEnter the second base of the trapezium:");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Please enter a valid number");
    let base2 :f64= input3.trim().parse().expect("Enter a valid number!");

    let area1 :f64 = (height / 2.0) * (base1 + base2);
    println!("\nThe area of the trapezium is {} square units.", area1);
}

//function for rhombus
fn rhombus(){
    println!("\nEnter the first diagonal of the rhombus:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Please enter a valid number");
    let diagonal1 :f64 = input1.trim().parse().expect("Enter a valid number!");

     println!("\nEnter the second diagonal of the rhombus:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Please enter a valid number");
    let diagonal2 :f64= input2.trim().parse().expect("Enter a valid number!");

    let area2 :f64 = 0.5 * diagonal1 * diagonal2;
    println!("\nThe area of the rhombus is {} square units.", area2);
}

//function for the parallelogram
fn parallelogram(){
    println!("\nEnter the base of the parallelogram:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Please enter a valid number");
    let base :f64 = input1.trim().parse().expect("Enter a valid number!");

    println!("\nEnter the height/altitude of the parallelogram:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Please enter a valid number");
    let altitude :f64 = input2.trim().parse().expect("Enter a valid number!");

    let area3: f64 = base * altitude;
    println!("\nThe area of the parallelogram is {} square units.", area3); 
}

// function for cube
fn cube(){
    println!("\nEnter the side of the cube:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Please enter a valid number");
    let side :f64 = input1.trim().parse().expect("Enter a valid number!");

    let surface_area: f64 = 6.0 * side * side;
    println!("\nThe surface area of the cube is {} square units.", surface_area);
}

//function for cylinder
fn cylinder(){
    println!("\nEnter the radius of the base of the cylinder:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Please enter a valid number");
    let radius :f64 = input1.trim().parse().expect("Enter a valid number!");

    println!("\nEnter the height of the cylinder:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Please enter a valid number");
    let height :f64 = input2.trim().parse().expect("Enter a valid number!");

    let volume = PI * radius * radius * height;
    println!("\nThe volume of the cylinder is {:.2} cube units.", volume);
}

//Welcoming the user
fn main() {
    println!("\nWelcome to the Shape Calculator!\n");
    println!("Here is a list of the available calculations for shapes:");
    println!("    1. Area of a Trapezium
    2. Area of a Rhombus
    3. Area of a Parallelogram
    4. Surface Area of a Cube
    5. Volume of a Cube");
    println!("What would you like to calculate today?
    Please select from 1 to 5:");

//accepting user's option
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Please select from 1 to 5");
    let option :u8  = input.trim().parse().expect("Enter a valid number from 1 to 5");

//calling functions with decisions
    if option == 1{
        trapezium();
    } else if option == 2{
        rhombus();
    } else if option == 3{
        parallelogram();
    } else if option == 4{
        cube();
    } else if option == 5{
        cylinder();
    } else {
        println!("Enter a number between 1 and 5");
    }
}