use std::io;

fn area_trapezium(h:f32,base1:f32,base2:f32){
    let trapezium:f32 = (h/2.0)*(base1 + base2);
    println!("The area of the trapezium is: {}", trapezium);
}

fn area_rhombus(d1:f32,d2:f32){
    let rhombus:f32 = 1.0/2.0 * d1 *d2;
    println!("The area of the rhombus is: {}", rhombus);
}

fn area_parallelogram(base:f32,alt:f32){
    let parallelogram:f32 = base * alt;
    println!("The area of the parallelogram is: {}", parallelogram);
}

fn surf_area_cube(length_of_side:f32){
    let surf_area_cube:f32 = 6.0 * length_of_side.powf(2.0);
    println!("The surface area of the cube is: {}", surf_area_cube);
}

fn get_pi()->f64 {
    let a:f64 = 22.0;
    let b:f64 = 7.0;
    let c:f64 = a/b;
    return c;
}

fn volume_cylinder(r:f64,height:f64){
    let volume_cylinder:f64 = get_pi()*r.powf(2.0)*height;
    println!("The volume of the cylinder is: {}", volume_cylinder);
}

fn main() {
loop{
    println!("\n\n\n          Select an equation         \n");
    println!("      T = Area of Trapezium          ");
    println!("      R = Area of Rhombus            ");
    println!("      P = Area of Parallelogram      ");
    println!("      SC = Surface Area of Cube      ");
    println!("      VCY = Volume of Cylinder       ");

    println!("\n\n Note: Use the codes for the equations \n");
    println!("What equation would you like to use? (T/R/P/SC/VCY):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let eqn = input.trim().to_lowercase().to_string();

    if eqn == "t" {
        let mut param1 = String::new();
        println!("Enter a height value:");
        io::stdin().read_line(&mut param1).expect("Failed to read input");
        let h:f32 = param1.trim().parse().expect("Failed to read input");

        let mut param2 = String::new();
        println!("Enter the first base value:");
        io::stdin().read_line(&mut param2).expect("Failed to read input");
        let base1:f32 = param2.trim().parse().expect("Failed to read input");

        let  mut param3 = String::new();
        println!("Enter the second base value:");
        io::stdin().read_line(&mut param3).expect("Failed to read input");
        let base2:f32 = param3.trim().parse().expect("Failed to read input");

        area_trapezium(h,base1,base2);
    }
    else if eqn == "r" {
        let mut param4 = String::new();
        println!("Enter the first diagonal value:");
        io::stdin().read_line(&mut param4).expect("Failed to read input");
        let d1:f32 = param4.trim().parse().expect("Failed to read input");

        let mut param5 = String::new();
        println!("Enter the second diagonal value:");
        io::stdin().read_line(&mut param5).expect("Failed to read input");
        let d2:f32 = param5.trim().parse().expect("Failed to read input");

        area_rhombus(d1,d2);

    }
    else if eqn == "p" {
        let mut param6 = String::new();
        println!("Enter the base value:");
        io::stdin().read_line(&mut param6).expect("Failed to read input");
        let base:f32 = param6.trim().parse().expect("Failed to read input");

        let mut param7 = String::new();
        println!("Enter the altitude value:");
        io::stdin().read_line(&mut param7).expect("Failed to read input");
        let alt:f32 = param7.trim().parse().expect("Failed to read input");

        area_parallelogram(base,alt);
    }
    else if eqn == "sc" {
        let mut param8 = String::new();
        println!("Enter the length of side value:");
        io::stdin().read_line(&mut param8).expect("Failed to read input");
        let length_of_side:f32 = param8.trim().parse().expect("Failed to read input");

        surf_area_cube(length_of_side);
}
    else if eqn == "vcy" {
        let mut param9 = String::new();
        println!("Enter the length of side value:");
        io::stdin().read_line(&mut param9).expect("Failed to read input");
        let r:f64 = param9.trim().parse().expect("Failed to read input");

        let mut param10 = String::new();
        println!("Enter the length of side value:");
        io::stdin().read_line(&mut param10).expect("Failed to read input");
        let height:f64 = param10.trim().parse().expect("Failed to read input");

        volume_cylinder(r,height);
}
else{
    println!("INVALID INPUT")
}
println!("Would you like to use another equation?  (y/ n)");
        let mut enter = String::new();
        io::stdin().read_line(&mut enter).expect("INVALID INPUT");
        let rept = enter.trim().to_lowercase().to_string();

        if rept == "y" || rept == "yes"
        {
            
        }
        else {
            break;
        }
}
}