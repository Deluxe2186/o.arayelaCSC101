struct Laptops{
    manufacturer:String,
    cost:u32,
    quantity:u32,
}

fn main() {
    
    let hp = Laptops{
        manufacturer:String::from("HP"),
        quantity:10,
        cost:650_000
    };
    let ibm = Laptops{
        manufacturer:String::from("IBM"),
        quantity:6,
        cost:755_000
    };
    let toshiba = Laptops{
        manufacturer:String::from("Toshiba"),
        quantity:10,
        cost:550_000
    };
    let dell = Laptops{
        manufacturer:String::from("Dell"),
        quantity:4,
        cost:850_000
    };

    display(&hp);
    display(&ibm);
    display(&toshiba);
    display(&dell);

    let total = 3 * (hp.cost + ibm.cost + toshiba.cost + dell.cost);

    println!("\nThe customer purchased 3 laptops \nfrom each brand making the total cost: \n\n${}", total);
}

fn display(x: &Laptops)-> &Laptops{
    println!("------------------------------");
    println!("Manufacturer: {}",x.manufacturer);
    println!("cost: ${}",x.cost);
    println!("quantity: {}",x.quantity);
    println!("------------------------------");
    return x;
}