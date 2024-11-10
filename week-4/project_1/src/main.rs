use std::io;

fn main() {
    println!("Program to determine the number of roots of a quadratic equation");

    //input values
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the value of a ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f64 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the value of b ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f64 = input2.trim().parse().expect("Not a valid number");

    println!("Enter the value of c ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f64 = input3.trim().parse().expect("Not a valid number");

    let discriminant:f64 = b.powf(2.0) - (4.0 * a * c);
    println!("The value of the discriminant is {}",discriminant);

    if discriminant > 0.0 {
        println!("The equation has 2 distinct roots");
            }
    else if discriminant == 0.0 {
        println!("There is exactly one root");
    }  
    else if discriminant < 0.0 {
        println!("There is no real root");
    }     



}
