use std::io;
use std::io::Write;

fn main() {
    let mut student_name: Vec<String> = Vec::new();
    let mut matriculation_no: Vec<String> = Vec::new();
    let mut department: Vec<String> = Vec::new();
    let mut level: Vec<u32> = Vec::new();

    println!("\n\nWelcome! This is PAU's Student Management Information System");
    println!("Kindly follow the instructions below:");

    loop {
        let mut name = String::new();
        println!("\n\nEnter student's fullname:");
        io::stdin().read_line(&mut name).expect("Failed to read input");
        student_name.push(name.trim().to_string());

        let mut matric_no = String::new();
        println!("\nEnter student's matriculation number:");
        io::stdin().read_line(&mut matric_no).expect("Failed to read input");
        matriculation_no.push(matric_no.trim().to_string());

        let mut dep = String::new();
        println!("\nEnter student's department:");
        io::stdin().read_line(&mut dep).expect("Failed to read input");
        department.push(dep.trim().to_string());

        let mut lvls = String::new();
        println!("\nEnter student's level:");
        io::stdin().read_line(&mut lvls).expect("Failed to read input");
        let lvl: u32 = lvls.trim().parse().expect("Invalid input");
        if lvl % 100 == 0 && lvl < 500 {
            level.push(lvl);
        } else {
            println!("Invalid level. Skipping...");
        }

        println!("Would you like to input another student's information? (y/n)");
        let mut enter = String::new();
        io::stdin().read_line(&mut enter).expect("INVALID INPUT");
        let response = enter.trim().to_lowercase();
        if response != "y" && response != "yes" {
            break;
        }
    }

    let mut file = std::fs::File::create("PAU-SMIS.txt").expect("Failed to create file");

    file.write_all(
        format!(
            "{:<30} {:<25} {:<25} {:<10}\n\n",
            "Student Name", "Matric.No", "Department", "Level"
        )
        .as_bytes(),
    )
    .expect("Failed to write header");

    for i in 0..student_name.len() {
        let a = &student_name[i];
        let b = &matriculation_no[i];
        let c = &department[i];
        let d = if i < level.len() { level[i] } else { 0 };

        file.write_all(
            format!("{:<30} {:<25} {:<25} {:<10}\n", a, b, c, d).as_bytes(),
        )
        .expect("Failed to write row");
    }

    println!("File has been created successfully.");
}