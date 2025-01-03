fn main() {
    
    // Array with data type (explicit integer datatype)
    let arr1:[i32;4] = [10,20,30,40];
    println!("\nArray with data type");
    println!("array is {:?}",arr1);
    println!("array size is:{}",arr1.len());

    // Array with data type (implicit float datatype)
    let arr2 = [10.4,20.7,30.4,40.9,51.2,72.2];
    println!("\nArray with data type");
    println!("array is {:?}",arr2);
    println!("array size is:{}",arr2.len());

    // Array with default values that create and
    // initializes all its elements with a default value of -1
    // Array with data type (explicit integer datatype)
    let arr3:[i32;8] = [-1;8];
    println!("\nArray with data type");
    println!("array is {:?}",arr3);
    println!("array size is:{}",arr3.len());
}