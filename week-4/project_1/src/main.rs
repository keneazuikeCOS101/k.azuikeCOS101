// rust program to find the roots of a quadratic eqution

use std::io;

fn main() {


    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    //user enters the a, b and c of the quadratic equation
    println!("Enter a");
    io::stdin().read_line(&mut input1).expect("Invalid input");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter b");
    io::stdin().read_line(&mut input2).expect("Invalid input");
    let b:f32= input2.trim().parse().expect("Not a valid number");

    println!("Enter c");
    io::stdin().read_line(&mut input3).expect("Invalid input");
    let c:f32= input3.trim().parse().expect("Not a valid number");


    //finding the discriminant
    let mut d:f32 = (b * b) - (4.0 * a * c);
    d = d.sqrt();

    //finding the roots
    let x1:f32 = (-b + d) / (2.0 * a);
    let x2:f32 = (-b - d) / (2.0 * a);
    println!("the roots of the equation are {} and {}", x1, x2);


    



}