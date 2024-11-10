use std::io;

fn main() {
    println!("Employee Incentive Calculator");
    println!("NOTE: Employees are classified as experienced if they have at least 5 years of work experience");

    //input age and years of experience
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter employee's age ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let age:f64 = input1.trim().parse().expect("Not a valid number");

    println!("Enter employee's years of experience ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let exp:f64 = input2.trim().parse().expect("Not a valid number");

    if age >= 40.0 && exp >= 5.0 {
        println!("This employees's annual incentive is N1,560,000");

    }
    
    else if age >= 30.0 && age < 40.0 && exp >= 5.0 {
        println!("This employee's annual incentive is N1,480,000");
    }

    else if age >= 28.0 && age <= 29.0 && exp >= 5.0 {
        println!("This employee's annual incentive is N1,400,000");
    }

    else if age < 28.0 && exp >= 5.0 {
        println!("This employee's annual incentive is N1,300,000");

    }    
    else if exp < 5.0 {
        println!("This employee's annual incentive is N100,000");
    }

}
