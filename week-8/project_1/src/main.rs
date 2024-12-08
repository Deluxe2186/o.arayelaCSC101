use std::io;

fn o(){
    let level = vec!["APS 1-2", "APS 3-4", "APS 5-7", "EL1 8-10", "EL2 11-13", "SES"];
    let office_admin = vec!["Intern", "Admininstrator", "Senior Administrator", "Office Manager", "Director", "CEO"];

    println!("\nHow many years of experience do you have?");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let exp:u32 = input2.trim().parse().expect("INVALID INPUT");

    if exp >=1 && exp <=2{
        println!("\n\nYou are an {} in position {}",office_admin[0],level[0])
    }
    else if exp >=3 && exp <=4{
        println!("\n\nYou are an {} in position {}",office_admin[1],level[1])
    }
    else if exp >=5 && exp <=7{
        println!("\n\nYou are a {} in position {}",office_admin[2],level[2])
    }
    else if exp >=8 && exp <=10{
        println!("\n\nYou are an {} in position {}",office_admin[3],level[3])
    }
    else if exp >=11 && exp <=13{
        println!("\n\nYou are a {} in position {}",office_admin[4],level[4])
    }
    else if exp >13{
        println!("\n\nYou are a {} in position {}",office_admin[5],level[5])
    }
    else {
        println!(" ");
    }
}

fn a(){
    let level = vec!["APS 1-2", "APS 3-4", "APS 5-7", "EL1 8-10", "EL2 11-13", "SES"];
    let academic = vec!["-", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];

    println!("\nHow many years of experience do you have?");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let exp:u32 = input2.trim().parse().expect("INVALID INPUT");

    if exp >=1 && exp <=2{
        println!("{}",academic[0])
    }
    else if exp >=3 && exp <=4{
        println!("\n\nYou are a {} in position {}",academic[1],level[1])
    }
    else if exp >=5 && exp <=7{
        println!("\n\nYou are a {} in position {}",academic[2],level[2])
    }
    else if exp >=8 && exp <=10{
        println!("\n\nYou are a {} in position {}",academic[3],level[3])
    }
    else if exp >=11 && exp <=13{
        println!("\n\nYou are a {} in position {}",academic[4],level[4])
    }
    else if exp >13{
        println!("\n\nYou are a {} in position {}",academic[5],level[5])
    }
    else {
        println!(" ");
    }
}

fn l(){
    let level = vec!["APS 1-2", "APS 3-4", "APS 5-7", "EL1 8-10", "EL2 11-13", "SES"];
    let lawyer = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];

    println!("\nHow many years of experience do you have?");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let exp:u32 = input2.trim().parse().expect("INVALID INPUT");

    if exp >=1 && exp <=2{
        println!("\n\nYou are a {} in position {}",lawyer[0],level[0])
    }
    else if exp >=3 && exp <=4{
        println!("\n\nYou are a {} in position {}",lawyer[1],level[1])
    }
    else if exp >=5 && exp <=7{
        println!("\n\nYou are an {} in position {}",lawyer[2],level[2])
    }
    else if exp >=8 && exp <=10{
        println!("\n\nYou are a {} in position {}",lawyer[3],level[3])
    }
    else if exp >=11 && exp <=13{
        println!("\n\nYou are a {} in position {}",lawyer[4],level[4])
    }
    else if exp >13{
        println!("\n\nYou are a {} in position {}",lawyer[5],level[5])
    }
    else {
        println!(" ");
    }
}

fn t(){
    let level = vec!["APS 1-2", "APS 3-4", "APS 5-7", "EL1 8-10", "EL2 11-13", "SES"];
    let teacher = vec!["Placement", "Classroom Teacher", "Snr Teacher", "Leading Teacher", "Deputy principal", "principal"];

    println!("\nHow many years of experience do you have?");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let exp:u32 = input2.trim().parse().expect("INVALID INPUT");

    if exp >=1 && exp <=2{
        println!("\n\nYou are a {} in position {}",teacher[0],level[0])
    }
    else if exp >=3 && exp <=4{
        println!("\n\nYou are a {} in position {}",teacher[1],level[1])
    }
    else if exp >=5 && exp <=7{
        println!("\n\nYou are a {} in position {}",teacher[2],level[2])
    }
    else if exp >=8 && exp <=10{
        println!("\n\nYou are a {} in position {}",teacher[3],level[3])
    }
    else if exp >=11 && exp <=13{
        println!("\n\nYou are a {} in position {}",teacher[4],level[4])
    }
    else if exp >13{
        println!("\n\nYou are a {} in position {}",teacher[5],level[5])
    }
    else {
        println!(" ");
    }
}
fn main() {

    println!("\n\n          Welcome to the Public Service APS level checker");
    println!("\n\n          Kindly enter what type of public servant \n\nYou are:\n");
    println!("                          (1.) Office Administrator");
    println!("                          (2.) Academic");
    println!("                          (3.) Lawyer");
    println!("                          (4.) Teacher\n\n");
    println!("               Note: choose the coresponding number (1/2/3/4)");

    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let occ:u32 = input1.trim().parse().expect("INVALID INPUT");

    if occ == 1 {
        o()
    }
    else if occ == 2 {
        a()
    }
    else if occ == 3 {
        l()
    }
    else if occ == 4 {
        t()
    }
    else {
        println!("INVALID INPUT")
    }
}