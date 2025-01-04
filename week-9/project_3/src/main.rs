use std::io::Write;
use std::io::Read;

fn main() {
    // Data vectors
    let headers = vec!["NAME OF COMMISIONER", "MINISTRY", "GEOPOLITICAL ZONE"];
    let commisioner_name = vec!["Aigbogun Alamba Daudu","Murtala Afeez Bendu","Okorocha Calistus Ogbona","Adewale Jimoh Akanbi","Osazuwa Faith Etieye"];
    let ministry = vec!["Internal Affairs","Justice","Defense","Power & Steel","Petroleum"];
    let geo_zone = vec!["South West","North East","South South","South West","South East"];

    // Creating the master file
    let mut master_file = std::fs::File::create("Master file.txt").expect("Failed to create file.");

    // Writing the headers
    let header_line = format!("{:<30} {:<20} {:<15}\n\n",headers[0], headers[1], headers[2]);
    
    master_file.write_all(header_line.as_bytes()).expect("Failed to write headers.");

    // Writing the rows
    for i in 0..5 {
        
        let row = format!("{:<30} {:<20} {:<15}\n",commisioner_name[i], ministry[i], geo_zone[i]);
        
        master_file.write_all(row.as_bytes()).expect("Failed to write rows.");
    }

    println!("\nMaster file created successfully.");

    println!("\n\nHere's the output:\n\n");

    //Reading the master file
    let mut file = std::fs::File::open("Master file.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);

    println!("\n\nCode written by Oluwayanmife Arayela.");
}