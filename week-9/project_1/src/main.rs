use std::io::Write;

fn main() {

    let mut file = std::fs::File::create("drinks.txt").expect("Failed to create file");

    file.write_all("Lager                 Stout                   Non-Alcoholic\n\n\n".as_bytes())
        .expect("Failed to write header");

    let lager = ["33 Export","Desperados","Goldberg","Gulder          ","Heineken","Star"];
    let stout = [" Legend","Turbo","  Williams"];
    let non_alcoholic = ["  Maltina","   Amstel Malta","Malta Gold","  Fayrouz"];

    // Loop through the largest list manually
    for i in 0..6 {
        let lager_item = if i < lager.len() { lager[i] } else { "" };
        let stout_item = if i < stout.len() { stout[i] } else { "" };
        let non_alcoholic_item = if i < non_alcoholic.len() { non_alcoholic[i] } else { "" };

        // Write each column with spaces for alignment
        file.write_all(lager_item.as_bytes()).expect("Failed to write lager");
        file.write_all("            ".as_bytes()).expect("Failed to write spaces");

        file.write_all(stout_item.as_bytes()).expect("Failed to write stout");
        file.write_all("                ".as_bytes()).expect("Failed to write spaces");

        file.write_all(non_alcoholic_item.as_bytes()).expect("Failed to write non-alcoholic");
        file.write_all("\n".as_bytes()).expect("Failed to write newline");
    }

    file.write_all("\n\nBy Oluwayanmife Arayela".as_bytes()).expect("Failed to write newline");
    println!("File written successfully.");
}